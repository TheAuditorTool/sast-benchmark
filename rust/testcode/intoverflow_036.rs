//! CWE-190: Addition uses saturating_add; result clamped to i64::MAX instead of wrapping.

// vuln-code-snippet start testcodeIntoverflow036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i64 = req.param("a").parse().unwrap_or(0);
    let b: i64 = req.param("b").parse().unwrap_or(0);
    let sum = a.saturating_add(b); // vuln-code-snippet target-line testcodeIntoverflow036
    super::shared::BenchmarkResponse::ok(&format!("sum={}", sum))
}
// vuln-code-snippet end testcodeIntoverflow036
