//! CWE-502: User-supplied field embedded via format! into JSON string before deserialization.

// vuln-code-snippet start testcodeDeser028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_data = req.param("user");
    let json = format!("{{\"user\":\"{}\"}}", user_data);
    let result = deserialize_to_value(&json); // vuln-code-snippet target-line testcodeDeser028
    super::shared::BenchmarkResponse::ok(&result)
}

fn deserialize_to_value(_json: &str) -> String {
    "serde_json::Value".to_string()
}
// vuln-code-snippet end testcodeDeser028
