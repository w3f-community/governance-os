| Number | Deliverable | Link | Notes |
|-|-|-|-|
| 0a. | License | [../LICENSE](../LICENSE) | We chose the Apache 2.0 license as initially agreed. |
| 0b. | Documentation | https://nucleistudio.github.io/governance-os/governance_os_node/index.html | The code contains inline rust documentation. Additionally, we use a github action to auto publish it. |
| 0c. | Testing Guide | [wave8_m2_testing.md](wave8_m2_testing.md) | The guide should cover any manual testing needs for you to confirm the functionalities of the pallets. When it comes to running our unit tests a simple `cargo test --all --all-features` should be enough. |
| 1. | `organizations` pallet | [../pallets/organizations](../pallets/organizations) | This pallet implements ways to manage organizations with custom parameters. It relies on the `bylaws` pallet for access control and `tokens` for voting. Runtime developers can implement their own voting systems that can be used by chain users. We provide an example, coin based, voting system in [../voting](../voting). Some organizations can be defined during genesis as well, this is what we do in our testing genesis configurations. |
| 3. | Demonstration Chain | [../runtime](../runtime) and [../node](../node) | We tried to keep the runtime as minimal as possible; it should be highly similar to the one provided during the first milestone with the addition of the `organizations` pallet along with our demonstration voting system. You may need to use the types available in [../types.json](../types.json) when testing. We have added many new types since milestone 1 so be sure to update those. |
| 4. | Dockerfile | [../Dockerfile](../Dockerfile) | You can build the container as usual. We also have a public image being built thanks to a little overnight [automation system](https://github.com/ETeissonniere/substrate-nodeops/), you can grab the public image under the name [`eteissonniere/governance-os`](https://hub.docker.com/r/eteissonniere/governance-os). |