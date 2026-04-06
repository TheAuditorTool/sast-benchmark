//! CWE-295: HTTP client configured to accept certificates regardless of validity.

// vuln-code-snippet start testcodeTlsverify001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    // Simulates: reqwest::ClientBuilder::new().danger_accept_invalid_certs(true).build()
    let client = reqwest_danger_client(true); // vuln-code-snippet target-line testcodeTlsverify001
    let result = format!("Fetched {} with client {}", url, client);
    super::shared::BenchmarkResponse::ok(&result)
}
fn reqwest_danger_client(accept_invalid: bool) -> String {
    format!("client(danger_accept_invalid_certs={})", accept_invalid)
}
// vuln-code-snippet end testcodeTlsverify001
