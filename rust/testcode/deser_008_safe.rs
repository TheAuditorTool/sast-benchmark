//! Insecure Deserialization True Negative — CWE-502
//! Content-Type check before deserialization. Only application/json
//! is accepted; other types are rejected before any parsing occurs.

// vuln-code-snippet start testcodeDeser008Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content_type = req.header("content-type");

    // SAFE: Content-Type verified before any deserialization
    if content_type != "application/json" { // vuln-code-snippet safe-line testcodeDeser008Safe
        return super::shared::BenchmarkResponse::bad_request("Only JSON accepted");
    }

    let body = req.body_str();
    let data: serde_json::Value = serde_json::from_str(&body)
        .unwrap_or(serde_json::Value::Null);

    super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", data))
}
// vuln-code-snippet end testcodeDeser008Safe
