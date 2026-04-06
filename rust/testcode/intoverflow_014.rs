//! CWE-190: Integer exponentiation with user-controlled exponent.

// vuln-code-snippet start testcodeIntoverflow014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let base: i32 = req.param("base").parse().unwrap_or(2);
    let exp: u32 = req.param("exp").parse().unwrap_or(1);
    let result = base.pow(exp); // vuln-code-snippet target-line testcodeIntoverflow014
    super::shared::BenchmarkResponse::ok(&format!("Power: {}", result))
}
// vuln-code-snippet end testcodeIntoverflow014
