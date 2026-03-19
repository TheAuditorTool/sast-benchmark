//! Insecure Deserialization True Positive — CWE-502
//! JSON with unknown fields accepted into untyped Value. Type confusion
//! allows attacker to inject unexpected fields that bypass logic.

// vuln-code-snippet start testcodeDeser003Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let json_input = req.body_str();

    // VULNERABLE: Untyped parse accepts any fields — type confusion possible
    let data: serde_json::Value = serde_json::from_str(&json_input) // vuln-code-snippet vuln-line testcodeDeser003Vulnerable
        .unwrap_or(serde_json::Value::Null);

    let role = data.get("role").and_then(|v| v.as_str()).unwrap_or("user");
    super::shared::BenchmarkResponse::ok(&format!("Role: {}", role))
}
// vuln-code-snippet end testcodeDeser003Vulnerable
