//! CWE-798: API key stored as string literal used in Authorization header.

// vuln-code-snippet start testcodeHardcodedcreds002
const API_KEY: &str = "sk-live-4eC39HqLyjWDarjtT1zdp7dc"; // vuln-code-snippet target-line testcodeHardcodedcreds002

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    // Simulates: reqwest::Client::new().get(&endpoint).header("Authorization", format!("Bearer {}", API_KEY))
    let result = format!("Calling {} with key {}", endpoint, &API_KEY[..8]);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds002
