//! CWE-190: Explicit bounds check on both operands before multiplication.

// vuln-code-snippet start testcodeIntoverflow038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i32 = req.param("a").parse().unwrap_or(0);
    let b: i32 = req.param("b").parse().unwrap_or(0);
    if a > 10000 || b > 10000 || a < -10000 || b < -10000 {
        return super::shared::BenchmarkResponse::bad_request("Operand out of bounds");
    }
    let result = a * b; // vuln-code-snippet target-line testcodeIntoverflow038
    super::shared::BenchmarkResponse::ok(&format!("result={}", result))
}
// vuln-code-snippet end testcodeIntoverflow038
