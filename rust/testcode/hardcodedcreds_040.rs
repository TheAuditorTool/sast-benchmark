//! CWE-798: API key retrieved via a helper function that reads only from environment.

// vuln-code-snippet start testcodeHardcodedcreds040
fn get_api_key() -> String {
    std::env::var("API_KEY").expect("API_KEY required")
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let key = get_api_key(); // vuln-code-snippet target-line testcodeHardcodedcreds040
    let result = format!("GET {} Authorization: Bearer {}", endpoint, key);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds040
