//! CWE-190: Only first multiplication operand validated; second operand can still cause overflow.

// vuln-code-snippet start testcodeIntoverflow033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i32 = req.param("a").parse().unwrap_or(0);
    let b: i32 = req.param("b").parse().unwrap_or(0);
    if a > 10000 {
        return super::shared::BenchmarkResponse::bad_request("a too large");
    }
    let result = a * b; // vuln-code-snippet target-line testcodeIntoverflow033
    super::shared::BenchmarkResponse::ok(&format!("result={}", result))
}
// vuln-code-snippet end testcodeIntoverflow033
