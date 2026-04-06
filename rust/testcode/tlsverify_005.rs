//! CWE-295: TLS client configured with empty root certificate store.

// vuln-code-snippet start testcodeTlsverify005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    // Simulates: rustls::ClientConfig with RootCertStore::empty()
    let config = rustls_empty_roots(); // vuln-code-snippet target-line testcodeTlsverify005
    super::shared::BenchmarkResponse::ok(&format!("Connected to {} with {}", url, config))
}
fn rustls_empty_roots() -> String {
    // Simulates: RootCertStore::empty() -- no trusted CAs
    "rustls(root_certs=0)".to_string()
}
// vuln-code-snippet end testcodeTlsverify005
