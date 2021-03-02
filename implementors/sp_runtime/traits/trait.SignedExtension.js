(function() {var implementors = {};
implementors["frame_system"] = [{"text":"impl&lt;T:&nbsp;Trait + Send + Sync&gt; SignedExtension for CheckGenesis&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Trait + Send + Sync&gt; SignedExtension for CheckMortality&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Trait&gt; SignedExtension for CheckNonce&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T::Call: Dispatchable&lt;Info = DispatchInfo&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Trait + Send + Sync&gt; SignedExtension for CheckSpecVersion&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Trait + Send + Sync&gt; SignedExtension for CheckTxVersion&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Trait + Send + Sync&gt; SignedExtension for CheckWeight&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T::Call: Dispatchable&lt;Info = DispatchInfo, PostInfo = PostDispatchInfo&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["pallet_transaction_payment"] = [{"text":"impl&lt;T:&nbsp;Trait + Send + Sync&gt; SignedExtension for ChargeTransactionPayment&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Trait&gt;::Currency as Currency&lt;&lt;T as Trait&gt;::AccountId&gt;&gt;::Balance: Send + Sync + From&lt;u64&gt; + FixedPointOperand,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::Call: Dispatchable&lt;Info = DispatchInfo, PostInfo = PostDispatchInfo&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["sp_runtime"] = [];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()