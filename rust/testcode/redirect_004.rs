//! CWE-601: Referer header used as post-login redirect target.

// vuln-code-snippet start testcodeRedirect004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");
    let referer = req.header("referer"); // vuln-code-snippet target-line testcodeRedirect004
    super::shared::BenchmarkResponse::ok(&format!("Location: {}", referer))
}
// vuln-code-snippet end testcodeRedirect004
