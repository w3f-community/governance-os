(function() {var implementors = {};
implementors["libp2p_core"] = [];
implementors["libp2p_deflate"] = [{"text":"impl&lt;C&gt; OutboundUpgrade&lt;C&gt; for DeflateConfig <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: AsyncRead + AsyncWrite,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["libp2p_floodsub"] = [{"text":"impl&lt;TSocket&gt; OutboundUpgrade&lt;TSocket&gt; for FloodsubRpc <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;TSocket: AsyncWrite + AsyncRead + Send + Unpin + 'static,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["libp2p_gossipsub"] = [{"text":"impl&lt;TSocket&gt; OutboundUpgrade&lt;TSocket&gt; for ProtocolConfig <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;TSocket: AsyncWrite + AsyncRead + Unpin + Send + 'static,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["libp2p_kad"] = [{"text":"impl&lt;C&gt; OutboundUpgrade&lt;C&gt; for KademliaProtocolConfig <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: AsyncRead + AsyncWrite + Unpin,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["libp2p_mplex"] = [{"text":"impl&lt;C&gt; OutboundUpgrade&lt;C&gt; for MplexConfig <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: AsyncRead + AsyncWrite + Unpin,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["libp2p_noise"] = [{"text":"impl&lt;T, C&gt; OutboundUpgrade&lt;T&gt; for NoiseConfig&lt;IX, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;NoiseConfig&lt;IX, C&gt;: UpgradeInfo,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AsyncRead + AsyncWrite + Unpin + Send + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Protocol&lt;C&gt; + AsRef&lt;[u8]&gt; + Zeroize + Send + 'static,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, C&gt; OutboundUpgrade&lt;T&gt; for NoiseConfig&lt;XX, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;NoiseConfig&lt;XX, C&gt;: UpgradeInfo,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AsyncRead + AsyncWrite + Unpin + Send + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Protocol&lt;C&gt; + AsRef&lt;[u8]&gt; + Zeroize + Send + 'static,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, C&gt; OutboundUpgrade&lt;T&gt; for NoiseConfig&lt;IK, C, (PublicKey&lt;C&gt;, PublicKey)&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;NoiseConfig&lt;IK, C, (PublicKey&lt;C&gt;, PublicKey)&gt;: UpgradeInfo,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AsyncRead + AsyncWrite + Unpin + Send + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Protocol&lt;C&gt; + AsRef&lt;[u8]&gt; + Zeroize + Send + 'static,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, P, C, R&gt; OutboundUpgrade&lt;T&gt; for NoiseAuthenticated&lt;P, C, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;NoiseConfig&lt;P, C, R&gt;: UpgradeInfo + OutboundUpgrade&lt;T, Output = (RemoteIdentity&lt;C&gt;, NoiseOutput&lt;T&gt;), Error = NoiseError&gt; + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;NoiseConfig&lt;P, C, R&gt; as OutboundUpgrade&lt;T&gt;&gt;::Future: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AsyncRead + AsyncWrite + Send + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Protocol&lt;C&gt; + AsRef&lt;[u8]&gt; + Zeroize + Send + 'static,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["libp2p_ping"] = [{"text":"impl OutboundUpgrade&lt;Negotiated&lt;SubstreamRef&lt;Arc&lt;StreamMuxerBox&gt;&gt;&gt;&gt; for Ping","synthetic":false,"types":[]}];
implementors["libp2p_plaintext"] = [{"text":"impl&lt;C&gt; OutboundUpgrade&lt;C&gt; for PlainText1Config","synthetic":false,"types":[]},{"text":"impl&lt;C&gt; OutboundUpgrade&lt;C&gt; for PlainText2Config <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: AsyncRead + AsyncWrite + Send + Unpin + 'static,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["libp2p_request_response"] = [{"text":"impl&lt;TCodec&gt; OutboundUpgrade&lt;Negotiated&lt;SubstreamRef&lt;Arc&lt;StreamMuxerBox&gt;&gt;&gt;&gt; for RequestProtocol&lt;TCodec&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;TCodec: RequestResponseCodec + Send + 'static,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["libp2p_yamux"] = [{"text":"impl&lt;C&gt; OutboundUpgrade&lt;C&gt; for Config <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: AsyncRead + AsyncWrite + Send + Unpin + 'static,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;C&gt; OutboundUpgrade&lt;C&gt; for LocalConfig <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: AsyncRead + AsyncWrite + Unpin + 'static,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()