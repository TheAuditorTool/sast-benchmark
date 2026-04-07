//! CWE-798: Function ignores its hardcoded argument and returns the real env var value.

// vuln-code-snippet start testcodeHardcodedcreds046
fn get_real_key(_placeholder: &str) -> String {
    std::env::var("API_KEY").unwrap_or_default()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let key = get_real_key("ignored-hardcoded-value"); // vuln-code-snippet target-line testcodeHardcodedcreds046
    let result = format!("GET {} Authorization: Bearer {}", endpoint, key);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds046
