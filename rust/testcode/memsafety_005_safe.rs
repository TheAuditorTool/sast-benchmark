//! Memory Safety True Negative — CWE-119
//! Box::new instead of raw allocation. Safe heap allocation managed by
//! Rust's ownership system, no manual free needed.

// vuln-code-snippet start testcodeMemsafety005Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: i64 = match req.param("value").parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid value"),
    };

    // SAFE: Box::new — safe heap allocation, auto-freed on drop
    let boxed = Box::new(val); // vuln-code-snippet safe-line testcodeMemsafety005Safe

    super::shared::BenchmarkResponse::ok(&format!("Boxed value: {}", boxed))
}
// vuln-code-snippet end testcodeMemsafety005Safe
