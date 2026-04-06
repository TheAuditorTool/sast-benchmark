//! CWE-601: Cookie value used as redirect destination after logout.

// vuln-code-snippet start testcodeRedirect005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dest = req.cookie("redirect_after"); // vuln-code-snippet target-line testcodeRedirect005
    super::shared::BenchmarkResponse::ok(&format!("Location: {}", dest))
}
// vuln-code-snippet end testcodeRedirect005
