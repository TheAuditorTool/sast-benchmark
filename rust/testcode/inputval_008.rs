//! CWE-20: Typed parse with proper error handling instead of unwrap.

// vuln-code-snippet start testcodeInputval008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let value_str = req.param("value");

    let value: i64 = match value_str.parse() { // vuln-code-snippet target-line testcodeInputval008
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Value must be a valid integer"),
    };

    super::shared::BenchmarkResponse::ok(&format!("Received value: {}", value))
}
// vuln-code-snippet end testcodeInputval008
