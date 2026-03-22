//! CWE-119: Cast with range check. Value validated before narrowing.

// vuln-code-snippet start testcodeMemsafety007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let big: i64 = match req.param("value").parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid value"),
    };


    if big < 0 || big > u16::MAX as i64 { // vuln-code-snippet target-line testcodeMemsafety007
        return super::shared::BenchmarkResponse::bad_request("Value out of u16 range");
    }
    let narrow = big as u16;

    super::shared::BenchmarkResponse::ok(&format!("Cast to u16: {}", narrow))
}
// vuln-code-snippet end testcodeMemsafety007
