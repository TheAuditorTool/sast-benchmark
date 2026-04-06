//! CWE-190: Left shift by user-controlled amount without bit-width check.

// vuln-code-snippet start testcodeIntoverflow012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let shift: u32 = req.param("shift").parse().unwrap_or(0);
    let result = 1u32 << shift; // vuln-code-snippet target-line testcodeIntoverflow012
    super::shared::BenchmarkResponse::ok(&format!("Shifted: {}", result))
}
// vuln-code-snippet end testcodeIntoverflow012
