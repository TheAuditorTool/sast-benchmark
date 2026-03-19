//! Insecure Deserialization True Negative — CWE-502
//! Schema validation after deserialization. Even if parse succeeds,
//! business rules reject invalid field combinations.

// vuln-code-snippet start testcodeDeser007Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let json_input = req.body_str();

    let data: serde_json::Value = serde_json::from_str(&json_input)
        .unwrap_or(serde_json::Value::Null);

    // SAFE: Schema validation rejects invalid payloads after parse
    if !validate_schema(&data) { // vuln-code-snippet safe-line testcodeDeser007Safe
        return super::shared::BenchmarkResponse::bad_request("Schema validation failed");
    }

    super::shared::BenchmarkResponse::ok(&format!("Valid: {}", data))
}

fn validate_schema(val: &serde_json::Value) -> bool {
    val.get("name").and_then(|v| v.as_str()).is_some()
        && val.get("email").and_then(|v| v.as_str()).is_some()
}
// vuln-code-snippet end testcodeDeser007Safe
