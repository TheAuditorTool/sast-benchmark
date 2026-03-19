//! Memory Safety True Negative — CWE-119
//! Vec with bounds-checked index. Uses .get() which returns Option,
//! preventing out-of-bounds access.

// vuln-code-snippet start testcodeMemsafety002Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let idx: usize = match req.param("index").parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid index"),
    };

    let data = vec![10u32, 20, 30, 40, 50];
    // SAFE: .get() returns Option — no panic or OOB on invalid index
    match data.get(idx) { // vuln-code-snippet safe-line testcodeMemsafety002Safe
        Some(val) => super::shared::BenchmarkResponse::ok(&format!("Value: {}", val)),
        None => super::shared::BenchmarkResponse::bad_request("Index out of range"),
    }
}
// vuln-code-snippet end testcodeMemsafety002Safe
