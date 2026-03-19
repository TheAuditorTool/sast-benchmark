//! Insecure Deserialization True Negative — CWE-502
//! Allowlisted types only. Deserialization target must be in a known
//! set of safe types; arbitrary type instantiation is blocked.

// vuln-code-snippet start testcodeDeser009Safe
const ALLOWED_TYPES: &[&str] = &["UserProfile", "Settings", "Preferences"];

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let type_name = req.param("type");
    let body = req.body_str();

    // SAFE: Only allowlisted types can be deserialized
    if !ALLOWED_TYPES.contains(&type_name.as_str()) { // vuln-code-snippet safe-line testcodeDeser009Safe
        return super::shared::BenchmarkResponse::bad_request("Type not allowed");
    }

    let data: serde_json::Value = serde_json::from_str(&body)
        .unwrap_or(serde_json::Value::Null);

    super::shared::BenchmarkResponse::ok(&format!("Type {}: {}", type_name, data))
}
// vuln-code-snippet end testcodeDeser009Safe
