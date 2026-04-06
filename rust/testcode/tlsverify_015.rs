//! CWE-295: rustls ClientConfig built with explicit root certificate store.

// vuln-code-snippet start testcodeTlsverify015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    // Simulates: rustls::ClientConfig::builder().with_root_certificates(roots).with_no_client_auth()
    let config = rustls_with_roots(); // vuln-code-snippet target-line testcodeTlsverify015
    super::shared::BenchmarkResponse::ok(&format!("Connected to {} with {}", url, config))
}
fn rustls_with_roots() -> String { "rustls(roots=custom_store,client_auth=none)".to_string() }
// vuln-code-snippet end testcodeTlsverify015
