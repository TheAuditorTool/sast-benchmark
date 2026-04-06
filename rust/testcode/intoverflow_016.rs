//! CWE-190: Cast from u64 to isize causing sign flip on large values.

// vuln-code-snippet start testcodeIntoverflow016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: u64 = req.param("value").parse().unwrap_or(0);
    let signed = val as isize; // vuln-code-snippet target-line testcodeIntoverflow016
    super::shared::BenchmarkResponse::ok(&format!("Signed: {}", signed))
}
// vuln-code-snippet end testcodeIntoverflow016
