(function() {var implementors = {};
implementors["hyper_rustls"] = [{"text":"impl&lt;T&gt; Service&lt;Uri&gt; for HttpsConnector&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Service&lt;Uri&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::Future: Send + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::Error: Into&lt;Box&lt;dyn Error + Send + Sync&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["tower_service"] = [];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()