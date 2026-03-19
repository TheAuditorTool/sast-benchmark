//! Memory Safety True Negative — CWE-119
//! Safe cast via `as` with range check first. Value validated to fit
//! in target type before the cast, preventing truncation.

// vuln-code-snippet start testcodeMemsafety007Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let big: i64 = match req.param("value").parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid value"),
    };

    // SAFE: Range check ensures value fits in u16 before cast
    if big < 0 || big > u16::MAX as i64 { // vuln-code-snippet safe-line testcodeMemsafety007Safe
        return super::shared::BenchmarkResponse::bad_request("Value out of u16 range");
    }
    let narrow = big as u16;

    super::shared::BenchmarkResponse::ok(&format!("Cast to u16: {}", narrow))
}
// vuln-code-snippet end testcodeMemsafety007Safe
