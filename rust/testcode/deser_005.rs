//! CWE-502: Size limit enforced before deserialization.

// vuln-code-snippet start testcodeDeser005
const MAX_BODY_SIZE: usize = 1_048_576; // 1 MB

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.body_str();

    if body.len() > MAX_BODY_SIZE { // vuln-code-snippet target-line testcodeDeser005
        return super::shared::BenchmarkResponse::bad_request("Payload too large");
    }

    let parsed: serde_json::Value = serde_json::from_str(&body)
        .unwrap_or(serde_json::Value::Null);

    super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", parsed))
}
// vuln-code-snippet end testcodeDeser005
