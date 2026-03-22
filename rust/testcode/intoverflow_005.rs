//! CWE-190: saturating_add clamps at MAX instead of wrapping.

// vuln-code-snippet start testcodeIntoverflow005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: u64 = req.param("a").parse().unwrap_or(0);
    let b: u64 = req.param("b").parse().unwrap_or(0);

    let result = a.saturating_add(b); // vuln-code-snippet target-line testcodeIntoverflow005

    super::shared::BenchmarkResponse::ok(&format!("Sum: {}", result))
}
// vuln-code-snippet end testcodeIntoverflow005
