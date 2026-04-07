//! CWE-190: Bit shift amount from user input not validated; shifting by 32+ is undefined behavior.

// vuln-code-snippet start testcodeIntoverflow029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let value: u32 = req.param("value").parse().unwrap_or(1);
    let shift: u32 = req.param("shift").parse().unwrap_or(0);
    let result = value << shift; // vuln-code-snippet target-line testcodeIntoverflow029
    super::shared::BenchmarkResponse::ok(&format!("result={}", result))
}
// vuln-code-snippet end testcodeIntoverflow029
