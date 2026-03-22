//! CWE-119: Box::new for heap allocation managed by ownership system.

// vuln-code-snippet start testcodeMemsafety005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: i64 = match req.param("value").parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid value"),
    };


    let boxed = Box::new(val); // vuln-code-snippet target-line testcodeMemsafety005

    super::shared::BenchmarkResponse::ok(&format!("Boxed value: {}", boxed))
}
// vuln-code-snippet end testcodeMemsafety005
