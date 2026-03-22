//! CWE-502: Content-Type header check before deserialization.
//! Content-Type is a client-supplied request header — an attacker
//! sets it to any value they choose.

// vuln-code-snippet start testcodeDeser008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content_type = req.header("content-type");

    // Content-Type is attacker-controlled — this check provides no security
    if content_type != "application/json" { // vuln-code-snippet target-line testcodeDeser008
        return super::shared::BenchmarkResponse::bad_request("Only JSON accepted");
    }

    let body = req.body_str();
    let data: serde_json::Value = serde_json::from_str(&body)
        .unwrap_or(serde_json::Value::Null);

    super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", data))
}
// vuln-code-snippet end testcodeDeser008
