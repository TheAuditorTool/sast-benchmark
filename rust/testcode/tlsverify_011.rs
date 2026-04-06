//! CWE-295: Default HTTP client build with certificate verification enabled.

// vuln-code-snippet start testcodeTlsverify011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    // Simulates: reqwest::Client::builder().build() -- defaults to verifying certs
    let client = reqwest_default_client(); // vuln-code-snippet target-line testcodeTlsverify011
    super::shared::BenchmarkResponse::ok(&format!("Fetched {} with {}", url, client))
}
fn reqwest_default_client() -> String { "reqwest(verify=true)".to_string() }
// vuln-code-snippet end testcodeTlsverify011
