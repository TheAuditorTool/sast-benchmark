//! CWE-190: Two user-supplied integers multiplied without overflow check.

// vuln-code-snippet start testcodeIntoverflow022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i32 = req.param("a").parse().unwrap_or(0);
    let b: i32 = req.param("b").parse().unwrap_or(0);
    let result = a * b; // vuln-code-snippet target-line testcodeIntoverflow022
    super::shared::BenchmarkResponse::ok(&format!("result={}", result))
}
// vuln-code-snippet end testcodeIntoverflow022
