//! Integer Overflow True Positive — CWE-190
//! Left shift by user-controlled amount. Shifting beyond bit width
//! is undefined in C and panics in debug Rust, wraps in release.

// vuln-code-snippet start testcodeIntoverflow002Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let base: u32 = req.param("base").parse().unwrap_or(1);
    let shift: u32 = req.param("shift").parse().unwrap_or(0);

    // VULNERABLE: Shift amount not bounded — overflow or panic
    let result = base << shift; // vuln-code-snippet vuln-line testcodeIntoverflow002Vulnerable

    super::shared::BenchmarkResponse::ok(&format!("Shifted: {}", result))
}
// vuln-code-snippet end testcodeIntoverflow002Vulnerable
