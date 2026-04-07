//! CWE-190: Multiplication uses checked_mul; overflow returns error instead of wrapping.

// vuln-code-snippet start testcodeIntoverflow035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i32 = req.param("a").parse().unwrap_or(0);
    let b: i32 = req.param("b").parse().unwrap_or(0);
    match a.checked_mul(b) {
        Some(result) => super::shared::BenchmarkResponse::ok(&format!("result={}", result)), // vuln-code-snippet target-line testcodeIntoverflow035
        None => super::shared::BenchmarkResponse::bad_request("Overflow"),
    }
}
// vuln-code-snippet end testcodeIntoverflow035
