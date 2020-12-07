/*
 * Copyright 2020 Nuclei Studio OÜ
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! This pallets creates and manages a set of organizations. An organization is linked
//! to a set of executors that can call its `apply_as` function to execute calls as if
//! it came from it.
//! For instance, a voting contract could be deployed and registered as an executor.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::{Dispatchable, Parameter},
    ensure,
    weights::GetDispatchInfo,
};
use frame_system::ensure_signed;
use governance_os_support::{
    ensure_not_err,
    traits::{Currencies, ReservableCurrencies, RoleManager, VotingHooks},
};
use sp_runtime::{
    traits::{AccountIdConversion, Hash, MaybeSerializeDeserialize, Member, StaticLookup},
    DispatchError, DispatchResult, ModuleId,
};
use sp_std::{boxed::Box, prelude::Vec};

mod details;
#[cfg(test)]
mod tests;

use details::{OrganizationDetails, Proposal};

pub trait RoleBuilder {
    type OrganizationId;
    type Role;

    /// Role for creating new organizations.
    fn create_organizations() -> Self::Role;

    /// This role gives the ability to execute calls as if they came
    /// from the organization address.
    fn apply_as_organization(org_id: &Self::OrganizationId) -> Self::Role;
}

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// Calls triggered from an organization.
    type Call: Parameter + GetDispatchInfo + Dispatchable<Origin = Self::Origin>;

    /// Pallet that is in charge of managing the roles based ACL.
    type RoleManager: RoleManager<AccountId = Self::AccountId>;

    /// This pallet relies on roles associated to a specific metadata so we need the runtime
    /// to provide some helper functions to build those so that we can keep the role definition
    /// code modular.
    type RoleBuilder: RoleBuilder<
        OrganizationId = Self::AccountId,
        Role = <RoleManagerOf<Self> as RoleManager>::Role,
    >;

    /// Pallet handling currencies. Used to represent voting weights.
    type Currencies: ReservableCurrencies<Self::AccountId>;

    /// The different kinds of voting system present inside the runtime.
    type VotingSystem: Parameter + Member + MaybeSerializeDeserialize;

    /// Some arbitrary data that can be added to proposals
    type ProposalMetadata: Parameter;

    /// Various hooks as implemented for each voting system.
    type VotingHooks: VotingHooks<
        AccountId = Self::AccountId,
        BlockNumber = Self::BlockNumber,
        Currencies = Self::Currencies,
        Data = Self::ProposalMetadata,
        OrganizationId = Self::AccountId,
        VotingSystem = Self::VotingSystem,
    >;
}

type BalanceOf<T> =
    <<T as Trait>::Currencies as Currencies<<T as frame_system::Trait>::AccountId>>::Balance;
type OrganizationDetailsOf<T> =
    OrganizationDetails<<T as frame_system::Trait>::AccountId, <T as Trait>::VotingSystem>;
pub type OrganizationsCounter = u32;
type ProposalIdOf<T> = <T as frame_system::Trait>::Hash;
type ProposalOf<T> = Proposal<
    Vec<u8>,
    <T as Trait>::ProposalMetadata,
    <T as frame_system::Trait>::AccountId,
    <T as Trait>::VotingSystem,
>;
type RoleBuilderOf<T> = <T as Trait>::RoleBuilder;
type RoleManagerOf<T> = <T as Trait>::RoleManager;

const ORGS_MODULE_ID: ModuleId = ModuleId(*b"gos/orgs");

decl_storage! {
    trait Store for Module<T: Trait> as Organizations {
        pub Counter get(fn counter): OrganizationsCounter = 0;
        pub Parameters get(fn parameters): map hasher(blake2_128_concat) T::AccountId => Option<OrganizationDetailsOf<T>>;
        pub Proposals get(fn proposals): map hasher(blake2_128_concat) ProposalIdOf<T> => Option<ProposalOf<T>>;
    }
    add_extra_genesis {
        config(organizations): Vec<OrganizationDetailsOf<T>>;
        build(|config: &GenesisConfig<T>| {
            config.organizations.iter().cloned().for_each(|params| {
                Module::<T>::do_create(params)
                    .expect("org creation in genesis block shall not fail")
            })
        })
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        OrganizationDetails = OrganizationDetailsOf<T>,
        ProposalId = ProposalIdOf<T>,
        Balance = BalanceOf<T>,
    {
        /// An organization was created with the following parameters. \[org. address, details\]
        OrganizationCreated(AccountId, OrganizationDetails),
        /// An organization executed a call. \[org. address, result\]
        OrganizationExecuted(AccountId, DispatchResult),
        /// An organization parameters have been modified. \[org. address, old details, new details\]
        OrganizationMutated(AccountId, OrganizationDetails, OrganizationDetails),
        /// A proposal has been submitted to an organization. \[org. address, proposal id\]
        ProposalSubmitted(AccountId, ProposalId),
        /// A proposal has been vetoed and removed from the queue of open proposals. \[proposal id\]
        ProposalVetoed(ProposalId),
        /// Somebody just voted on a proposal. \[proposal id, voter, power, support\]
        ProposalVoteCasted(ProposalId, AccountId, Balance, bool),
        /// A proposal has been executed with the following result. \[proposal id, result\]
        ProposalExecuted(ProposalId, DispatchResult),
        /// A proposal was closed. \[proposal id, wether it passed or not\]
        ProposalClosed(ProposalId, bool),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// We have created the maximum number of organizations, a runtime upgrade may
        /// be necessary.
        CounterOverflow,
        /// This call can only be executed by an organization.
        NotAnOrganization,
        /// A similar proposal already exists.
        ProposalDuplicate,
        /// The proposal is not linked to this organization.
        ProposalNotForOrganization,
        /// The proposal does not exists, maybe it was already closed.
        ProposalNotFound,
        /// This proposal can not be closed yet. It probably needs to wait for more votes.
        ProposalCanNotBeClosed,
        /// The proposal code couldn't be decoded for some reason. This isn't expected to ever
        /// happen and thus should be reported upstream.
        ProposalDecodingFailure,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Create an organization with the given parameters. An event will be triggered with
        /// the organization's address.
        #[weight = 0]
        fn create(origin, details: OrganizationDetailsOf<T>) {
            RoleManagerOf::<T>::ensure_has_role(origin, RoleBuilderOf::<T>::create_organizations())?;
            Self::do_create(details)?;
        }

        /// Trigger a call as if it came from the organization itself.
        #[weight = 0]
        fn apply_as(origin, org_id: <T::Lookup as StaticLookup>::Source, call: Box<<T as Trait>::Call>) {
            let target_org_id = T::Lookup::lookup(org_id)?;
            RoleManagerOf::<T>::ensure_has_role(origin, RoleBuilderOf::<T>::apply_as_organization(&target_org_id))?;

            let res = call.dispatch(frame_system::RawOrigin::Signed(target_org_id.clone()).into());
            Self::deposit_event(RawEvent::OrganizationExecuted(target_org_id, res.map(|_| ()).map_err(|e| e.error)));
        }

        /// Mutate an organization to use the new parameters. Only an organization can call this on itself.
        #[weight = 0]
        fn mutate(origin, new_details: OrganizationDetailsOf<T>) {
            let (org_id, old_details) = Self::ensure_org(origin)?;

            // Make sure everything is sorted for optimization purposes
            let mut new_details = new_details;
            new_details.sort();

            Self::try_run_on_changes(old_details.executors.as_slice(), new_details.executors.as_slice(), |old_account| {
                RoleManagerOf::<T>::revoke_role(Some(old_account), RoleBuilderOf::<T>::apply_as_organization(&org_id))
            }, |new_account| {
                RoleManagerOf::<T>::grant_role(Some(new_account), RoleBuilderOf::<T>::apply_as_organization(&org_id))
            })?;
            Parameters::<T>::insert(&org_id, new_details.clone());

            Self::deposit_event(RawEvent::OrganizationMutated(org_id, old_details, new_details));
        }

        /// Create a proposal for a given organization
        #[weight = 0]
        fn create_proposal(origin, org_id: <T::Lookup as StaticLookup>::Source, call: Box<<T as Trait>::Call>) {
            let who = ensure_signed(origin)?;
            let target_org_id = T::Lookup::lookup(org_id)?;
            let details = Self::try_get_parameters(&target_org_id)?;
            let proposal_id = Self::proposal_id(&target_org_id, call.clone());
            if Proposals::<T>::contains_key(proposal_id) {
                return Err(Error::<T>::ProposalDuplicate.into());
            }

            let (maybe_hook_sucessful, additional_data) = T::VotingHooks::on_create_proposal(details.clone().voting, &who, frame_system::Module::<T>::block_number());
            ensure_not_err!(maybe_hook_sucessful);

            Proposals::<T>::insert(&proposal_id, Proposal{
                org: target_org_id.clone(),
                call: call.encode(),
                metadata: additional_data,
                // Not only does this save us future read weights but it also cover
                // the case where an org change voting systems but still has pending
                // proposals.
                voting: details.voting,
            });

            Self::deposit_event(RawEvent::ProposalSubmitted(target_org_id, proposal_id));
        }

        /// Remove a proposal from the batch of active ones. Has to be called by the organization itself,
        /// typically this could come from an 'apply_as' or a separate vote.
        #[weight = 0]
        fn veto_proposal(origin, proposal_id: ProposalIdOf<T>) {
            let (org_id, _details) = Self::ensure_org(origin)?;
            let proposal = Self::try_get_proposal(proposal_id)?;
            ensure!(proposal.org == org_id, Error::<T>::ProposalNotForOrganization);

            T::VotingHooks::on_veto_proposal(proposal.voting, proposal.metadata)?;
            Proposals::<T>::remove(proposal_id);

            Self::deposit_event(RawEvent::ProposalVetoed(proposal_id));
        }

        /// Vote for or against a given proposal. The caller can choose how much voting power is dedicated
        /// to it via the `power` parameter.
        #[weight = 0]
        fn decide_on_proposal(origin, proposal_id: ProposalIdOf<T>, power: BalanceOf<T>, in_support: bool) {
            let voter = ensure_signed(origin)?;
            let mut proposal = Self::try_get_proposal(proposal_id)?;
            let (maybe_hook_sucessful, maybe_new_metadata) = T::VotingHooks::on_decide_on_proposal(proposal.clone().voting, proposal.clone().metadata, &voter, power, in_support);
            ensure_not_err!(maybe_hook_sucessful);
            if maybe_new_metadata != proposal.metadata {
                // Save the new metadata, overwriting the previous one
                proposal.metadata = maybe_new_metadata;
                Proposals::<T>::insert(proposal_id, proposal);
            }

            Self::deposit_event(RawEvent::ProposalVoteCasted(proposal_id, voter, power, in_support));
        }

        /// If a proposal passed or failed but is not longer awaiting or waiting for votes it can be closed. Closing
        /// a proposal means executing it if it passed, freeing all funds locked and erasing it from the local storage.
        #[weight = 0]
        fn close_proposal(origin, proposal_id: ProposalIdOf<T>) {
            let _ = ensure_signed(origin)?;
            let proposal = Self::try_get_proposal(proposal_id)?;
            ensure!(T::VotingHooks::can_close(proposal.clone().voting, proposal.clone().metadata, frame_system::Module::<T>::block_number()), Error::<T>::ProposalCanNotBeClosed);

            let passing = T::VotingHooks::passing(proposal.clone().voting, proposal.clone().metadata);
            T::VotingHooks::on_close_proposal(proposal.clone().voting, proposal.clone().metadata, passing)?;
            if passing {
                let decoded_call = <T as Trait>::Call::decode(&mut &proposal.clone().call[..]).map_err(|_| Error::<T>::ProposalDecodingFailure)?;
                let res = decoded_call.dispatch(frame_system::RawOrigin::Signed(proposal.clone().org).into());
                Self::deposit_event(RawEvent::ProposalExecuted(proposal_id, res.map(|_| ()).map_err(|e| e.error)));
            }
            Proposals::<T>::remove(proposal_id);

            Self::deposit_event(RawEvent::ProposalClosed(proposal_id, passing));
        }
    }
}

impl<T: Trait> Module<T> {
    /// A handy helper functions to run two functions on what elements were removed from a
    /// slice and on the added elements. Should be pretty useful when trying to limit
    /// database read and writes since we execute the functions only on the changed elements.
    fn try_run_on_changes<Elem: Ord>(
        old_vec: &[Elem],
        new_vec: &[Elem],
        on_old: impl Fn(&Elem) -> DispatchResult,
        on_new: impl Fn(&Elem) -> DispatchResult,
    ) -> DispatchResult {
        Self::try_run_if_not_in_right(old_vec, new_vec, on_old)
            .and_then(|_| Self::try_run_if_not_in_right(new_vec, old_vec, on_new))
    }

    /// Run `to_run` on every elent that is in `left` but not in `right`.
    fn try_run_if_not_in_right<Elem: Ord>(
        left: &[Elem],
        right: &[Elem],
        to_run: impl Fn(&Elem) -> DispatchResult,
    ) -> DispatchResult {
        left.into_iter().try_for_each(|elem| {
            if right.binary_search(&elem).is_err() {
                return to_run(elem);
            }

            Ok(())
        })
    }

    /// Given any counter return the associated organization id
    fn org_id_for(counter: u32) -> T::AccountId {
        ORGS_MODULE_ID.into_sub_account(counter)
    }

    /// Makes sure that the `origin` is a registered organization
    fn ensure_org(
        origin: T::Origin,
    ) -> Result<(T::AccountId, OrganizationDetailsOf<T>), DispatchError> {
        match ensure_signed(origin) {
            Err(e) => Err(e.into()),
            Ok(maybe_org_id) => Ok((
                maybe_org_id.clone(),
                Self::try_get_parameters(&maybe_org_id)?,
            )),
        }
    }

    /// Fetch an org details or error
    fn try_get_parameters(
        org_id: &T::AccountId,
    ) -> Result<OrganizationDetailsOf<T>, DispatchError> {
        match Parameters::<T>::get(org_id) {
            Some(details) => Ok(details),
            None => Err(Error::<T>::NotAnOrganization.into()),
        }
    }

    /// Fetch a proposal details or error
    fn try_get_proposal(proposal_id: ProposalIdOf<T>) -> Result<ProposalOf<T>, DispatchError> {
        match Proposals::<T>::get(proposal_id) {
            Some(proposal) => Ok(proposal),
            None => Err(Error::<T>::ProposalNotFound.into()),
        }
    }

    /// Hash the block number, org id and proposal together to generate its id
    fn proposal_id(org_id: &T::AccountId, proposal: Box<<T as Trait>::Call>) -> ProposalIdOf<T> {
        T::Hashing::hash_of(&[
            frame_system::Module::<T>::block_number().encode(),
            org_id.encode(),
            proposal.encode(),
        ])
    }

    fn do_create(details: OrganizationDetailsOf<T>) -> DispatchResult {
        let counter = Self::counter();
        let new_counter = counter.checked_add(1).ok_or(Error::<T>::CounterOverflow)?;
        let org_id = Self::org_id_for(counter);

        // Sorting details allows us to be more efficient when updating them later on.
        let mut details = details;
        details.sort();

        // We first write the counter so that even if the calls below fail we will always regenerate a new and
        // different organization id.
        details.executors.iter().try_for_each(|account| {
            RoleManagerOf::<T>::grant_role(
                Some(&account),
                RoleBuilderOf::<T>::apply_as_organization(&org_id),
            )
        })?;
        Counter::put(new_counter);
        Parameters::<T>::insert(&org_id, details.clone());

        Self::deposit_event(RawEvent::OrganizationCreated(org_id, details));
        Ok(())
    }
}
