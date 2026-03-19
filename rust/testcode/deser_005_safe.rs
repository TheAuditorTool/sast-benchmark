//! Insecure Deserialization True Negative — CWE-502
//! Size limit enforced before deserialization. Payloads over 1MB
//! are rejected, preventing YAML bombs and memory exhaustion.

// vuln-code-snippet start testcodeDeser005Safe
const MAX_BODY_SIZE: usize = 1_048_576; // 1 MB

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.body_str();

    // SAFE: Size limit before deserialize — rejects oversized payloads
    if body.len() > MAX_BODY_SIZE { // vuln-code-snippet safe-line testcodeDeser005Safe
        return super::shared::BenchmarkResponse::bad_request("Payload too large");
    }

    let parsed: serde_json::Value = serde_json::from_str(&body)
        .unwrap_or(serde_json::Value::Null);

    super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", parsed))
}
// vuln-code-snippet end testcodeDeser005Safe
