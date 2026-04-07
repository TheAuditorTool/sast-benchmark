//! CWE-190: Bit shift amount clamped to valid range 0..31 before shifting.

// vuln-code-snippet start testcodeIntoverflow041
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let value: u32 = req.param("value").parse().unwrap_or(1);
    let shift: u32 = req.param("shift").parse().unwrap_or(0).min(31);
    let result = value << shift; // vuln-code-snippet target-line testcodeIntoverflow041
    super::shared::BenchmarkResponse::ok(&format!("result={}", result))
}
// vuln-code-snippet end testcodeIntoverflow041
