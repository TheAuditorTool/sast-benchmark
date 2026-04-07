//! CWE-190: User-supplied u64 cast to i32 truncates and may produce negative result.

// vuln-code-snippet start testcodeIntoverflow026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: u64 = req.param("value").parse().unwrap_or(0);
    let signed = val as i32; // vuln-code-snippet target-line testcodeIntoverflow026
    super::shared::BenchmarkResponse::ok(&format!("signed={}", signed))
}
// vuln-code-snippet end testcodeIntoverflow026
