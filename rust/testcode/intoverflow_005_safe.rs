//! Integer Overflow True Negative — CWE-190
//! saturating_add clamps at MAX instead of wrapping. Result is always
//! a valid value, never silently wrong.

// vuln-code-snippet start testcodeIntoverflow005Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: u64 = req.param("a").parse().unwrap_or(0);
    let b: u64 = req.param("b").parse().unwrap_or(0);

    // SAFE: saturating_add clamps at u64::MAX instead of wrapping
    let result = a.saturating_add(b); // vuln-code-snippet safe-line testcodeIntoverflow005Safe

    super::shared::BenchmarkResponse::ok(&format!("Sum: {}", result))
}
// vuln-code-snippet end testcodeIntoverflow005Safe
