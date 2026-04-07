//! CWE-190: Division result cast from i64 to i16 truncates; potential data loss.

// vuln-code-snippet start testcodeIntoverflow034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let numerator: i64 = req.param("num").parse().unwrap_or(0);
    let denominator: i64 = req.param("den").parse().unwrap_or(1);
    let result = (numerator / denominator) as i16; // vuln-code-snippet target-line testcodeIntoverflow034
    super::shared::BenchmarkResponse::ok(&format!("result={}", result))
}
// vuln-code-snippet end testcodeIntoverflow034
