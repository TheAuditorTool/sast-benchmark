//! CWE-502: JSON with unknown fields accepted into untyped Value.

// vuln-code-snippet start testcodeDeser003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let json_input = req.body_str();

    let data: serde_json::Value = serde_json::from_str(&json_input) // vuln-code-snippet target-line testcodeDeser003
        .unwrap_or(serde_json::Value::Null);

    let role = data.get("role").and_then(|v| v.as_str()).unwrap_or("user");
    super::shared::BenchmarkResponse::ok(&format!("Role: {}", role))
}
// vuln-code-snippet end testcodeDeser003
