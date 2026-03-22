//! CWE-190: Unchecked multiply of two user-supplied i32 values.

// vuln-code-snippet start testcodeIntoverflow001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i32 = req.param("a").parse().unwrap_or(0);
    let b: i32 = req.param("b").parse().unwrap_or(0);

    let result = a * b; // vuln-code-snippet target-line testcodeIntoverflow001

    super::shared::BenchmarkResponse::ok(&format!("Product: {}", result))
}
// vuln-code-snippet end testcodeIntoverflow001
