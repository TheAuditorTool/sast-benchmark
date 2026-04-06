//! CWE-295: Custom CA certificate added to client trust store.

// vuln-code-snippet start testcodeTlsverify012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    // Simulates: reqwest::Client::builder().add_root_certificate(custom_ca).build()
    let client = reqwest_with_custom_ca(); // vuln-code-snippet target-line testcodeTlsverify012
    super::shared::BenchmarkResponse::ok(&format!("Fetched {} with {}", url, client))
}
fn reqwest_with_custom_ca() -> String { "reqwest(custom_ca=internal_ca.pem)".to_string() }
// vuln-code-snippet end testcodeTlsverify012
