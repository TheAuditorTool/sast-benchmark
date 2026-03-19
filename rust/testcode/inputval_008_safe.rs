//! Input Validation True Negative — CWE-20
//! Typed parse with proper error handling instead of unwrap.

// vuln-code-snippet start testcodeInputval008Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let value_str = req.param("value");

    // SAFE: Parse with match — returns error on invalid input instead of panicking
    let value: i64 = match value_str.parse() { // vuln-code-snippet safe-line testcodeInputval008Safe
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Value must be a valid integer"),
    };

    super::shared::BenchmarkResponse::ok(&format!("Received value: {}", value))
}
// vuln-code-snippet end testcodeInputval008Safe
