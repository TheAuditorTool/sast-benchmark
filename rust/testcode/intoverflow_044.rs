//! CWE-190: User-supplied multiplier unconditionally overwritten with safe constant value.

// vuln-code-snippet start testcodeIntoverflow044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i32 = req.param("a").parse().unwrap_or(0);
    let mut b: i32 = req.param("b").parse().unwrap_or(0);
    b = 2;
    let result = a * b; // vuln-code-snippet target-line testcodeIntoverflow044
    super::shared::BenchmarkResponse::ok(&format!("result={}", result))
}
// vuln-code-snippet end testcodeIntoverflow044
