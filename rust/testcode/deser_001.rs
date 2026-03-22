//! CWE-502: Raw bytes deserialized via bincode without type validation.

// vuln-code-snippet start testcodeDeser001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw_bytes = req.body_str().into_bytes();

    let obj: serde_json::Value = bincode_deserialize(&raw_bytes); // vuln-code-snippet target-line testcodeDeser001

    super::shared::BenchmarkResponse::ok(&format!("Deserialized: {}", obj))
}

fn bincode_deserialize(data: &[u8]) -> serde_json::Value {
    // Simulates bincode::deserialize — trusts input bytes completely
    serde_json::from_slice(data).unwrap_or(serde_json::Value::Null)
}
// vuln-code-snippet end testcodeDeser001
