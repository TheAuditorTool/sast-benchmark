//! CWE-190: Negation of user-supplied i32 overflows when value is i32::MIN.

// vuln-code-snippet start testcodeIntoverflow030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: i32 = req.param("value").parse().unwrap_or(0);
    let negated = -val; // vuln-code-snippet target-line testcodeIntoverflow030
    super::shared::BenchmarkResponse::ok(&format!("negated={}", negated))
}
// vuln-code-snippet end testcodeIntoverflow030
