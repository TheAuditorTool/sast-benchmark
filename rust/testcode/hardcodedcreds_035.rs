//! CWE-798: API key sourced from environment variable for use in request header.

// vuln-code-snippet start testcodeHardcodedcreds035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let api_key = std::env::var("API_KEY").unwrap_or_else(|_| String::new()); // vuln-code-snippet target-line testcodeHardcodedcreds035
    let result = format!("GET {} Authorization: Bearer {}", endpoint, api_key);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds035
