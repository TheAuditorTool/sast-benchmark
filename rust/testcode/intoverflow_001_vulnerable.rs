//! Integer Overflow True Positive — CWE-190
//! Unchecked multiply of two user-supplied i32 values. In release mode
//! Rust wraps on overflow, producing silently wrong results.

// vuln-code-snippet start testcodeIntoverflow001Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i32 = req.param("a").parse().unwrap_or(0);
    let b: i32 = req.param("b").parse().unwrap_or(0);

    // VULNERABLE: Unchecked multiply — wraps silently in release mode
    let result = a * b; // vuln-code-snippet vuln-line testcodeIntoverflow001Vulnerable

    super::shared::BenchmarkResponse::ok(&format!("Product: {}", result))
}
// vuln-code-snippet end testcodeIntoverflow001Vulnerable
