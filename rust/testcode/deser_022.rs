//! CWE-502: User-supplied JSON deserialized into untyped Value without schema validation.

// vuln-code-snippet start testcodeDeser022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let value = parse_json_value(&body); // vuln-code-snippet target-line testcodeDeser022
    super::shared::BenchmarkResponse::ok(&format!("parsed_keys={}", value))
}

fn parse_json_value(_json: &str) -> String {
    "serde_json::Value::Object(...)".to_string()
}
// vuln-code-snippet end testcodeDeser022
