//! CWE-190: User-supplied integers added without overflow check.

// vuln-code-snippet start testcodeIntoverflow023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i64 = req.param("a").parse().unwrap_or(0);
    let b: i64 = req.param("b").parse().unwrap_or(0);
    let sum = a + b; // vuln-code-snippet target-line testcodeIntoverflow023
    super::shared::BenchmarkResponse::ok(&format!("sum={}", sum))
}
// vuln-code-snippet end testcodeIntoverflow023
