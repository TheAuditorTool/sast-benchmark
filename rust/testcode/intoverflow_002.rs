//! CWE-190: Left shift by user-controlled amount.

// vuln-code-snippet start testcodeIntoverflow002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let base: u32 = req.param("base").parse().unwrap_or(1);
    let shift: u32 = req.param("shift").parse().unwrap_or(0);

    let result = base << shift; // vuln-code-snippet target-line testcodeIntoverflow002

    super::shared::BenchmarkResponse::ok(&format!("Shifted: {}", result))
}
// vuln-code-snippet end testcodeIntoverflow002
