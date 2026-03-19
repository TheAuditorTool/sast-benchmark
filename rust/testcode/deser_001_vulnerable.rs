//! Insecure Deserialization True Positive — CWE-502
//! Raw bytes deserialized via simulated bincode without type validation.
//! Attacker controls serialized payload, can trigger arbitrary behavior.

// vuln-code-snippet start testcodeDeser001Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw_bytes = req.body_str().into_bytes();

    // VULNERABLE: Raw bytes deserialized without type safety or validation
    let obj: serde_json::Value = bincode_deserialize(&raw_bytes); // vuln-code-snippet vuln-line testcodeDeser001Vulnerable

    super::shared::BenchmarkResponse::ok(&format!("Deserialized: {}", obj))
}

fn bincode_deserialize(data: &[u8]) -> serde_json::Value {
    // Simulates bincode::deserialize — trusts input bytes completely
    serde_json::from_slice(data).unwrap_or(serde_json::Value::Null)
}
// vuln-code-snippet end testcodeDeser001Vulnerable
