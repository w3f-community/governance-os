(function() {var implementors = {};
implementors["sc_consensus_aura"] = [{"text":"impl&lt;Block:&nbsp;BlockT, C, I, P&gt; BlockImport&lt;Block&gt; for AuraBlockImport&lt;Block, C, I, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: BlockImport&lt;Block, Transaction = TransactionFor&lt;C, Block&gt;&gt; + Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Error: Into&lt;ConsensusError&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: HeaderBackend&lt;Block&gt; + ProvideRuntimeApi&lt;Block&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Pair + Send + Sync + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;P::Public: Clone + Eq + Send + Sync + Hash + Debug + Encode + Decode,<br>&nbsp;&nbsp;&nbsp;&nbsp;P::Signature: Encode + Decode,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["sc_finality_grandpa"] = [{"text":"impl&lt;BE, Block:&nbsp;BlockT, Client, SC&gt; BlockImport&lt;Block&gt; for GrandpaBlockImport&lt;BE, Block, Client, SC&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;NumberFor&lt;Block&gt;: BlockNumberOps,<br>&nbsp;&nbsp;&nbsp;&nbsp;DigestFor&lt;Block&gt;: Encode,<br>&nbsp;&nbsp;&nbsp;&nbsp;BE: Backend&lt;Block&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Client: ClientForGrandpa&lt;Block, BE&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&amp;'a Client: BlockImport&lt;Block, Error = ConsensusError, Transaction = TransactionFor&lt;Client, Block&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;BE, Block:&nbsp;BlockT, Client&gt; BlockImport&lt;Block&gt; for GrandpaLightBlockImport&lt;BE, Block, Client&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;NumberFor&lt;Block&gt;: BlockNumberOps,<br>&nbsp;&nbsp;&nbsp;&nbsp;DigestFor&lt;Block&gt;: Encode,<br>&nbsp;&nbsp;&nbsp;&nbsp;BE: Backend&lt;Block&gt; + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;&amp;'a Client: HeaderBackend&lt;Block&gt; + BlockImport&lt;Block, Error = ConsensusError, Transaction = TransactionFor&lt;BE, Block&gt;&gt; + Finalizer&lt;Block, BE&gt; + AuxStore,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["sp_consensus"] = [];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()