//! CWE-798: HashMap contains a fake placeholder entry but the used key is sourced from env.

// vuln-code-snippet start testcodeHardcodedcreds044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let mut creds = std::collections::HashMap::new();
    creds.insert("hardcoded", "fake-key-placeholder".to_string());
    creds.insert("env_key", std::env::var("API_KEY").unwrap_or_default());
    let key = creds.get("env_key").unwrap(); // vuln-code-snippet target-line testcodeHardcodedcreds044
    let result = format!("GET {} Authorization: Bearer {}", endpoint, key);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds044
