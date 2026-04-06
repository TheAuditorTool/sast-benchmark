//! CWE-295: HTTPS client using rustls dangerous() API to bypass verification.

// vuln-code-snippet start testcodeTlsverify009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    // Simulates: rustls::ClientConfig::dangerous().with_custom_certificate_verifier(NoVerifier)
    let config = rustls_dangerous_config(); // vuln-code-snippet target-line testcodeTlsverify009
    super::shared::BenchmarkResponse::ok(&format!("Connected to {} with {}", url, config))
}
fn rustls_dangerous_config() -> String { "rustls(dangerous=true)".to_string() }
// vuln-code-snippet end testcodeTlsverify009
