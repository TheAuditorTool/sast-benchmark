//! CWE-601: User-controlled path segment in URL allowing ../ traversal to external host.

// vuln-code-snippet start testcodeRedirect010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_path = req.param("path");
    let redirect = format!("/app/{}", user_path); // vuln-code-snippet target-line testcodeRedirect010
    super::shared::BenchmarkResponse::ok(&format!("Location: {}", redirect))
}
// vuln-code-snippet end testcodeRedirect010
