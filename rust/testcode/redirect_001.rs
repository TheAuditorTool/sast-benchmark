//! CWE-601: User-supplied URL placed directly in HTTP Location header for redirect.

// vuln-code-snippet start testcodeRedirect001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_url = req.param("url");
    let header = format!("Location: {}", user_url); // vuln-code-snippet target-line testcodeRedirect001
    super::shared::BenchmarkResponse::ok(&format!("302 Redirect\n{}", header))
}
// vuln-code-snippet end testcodeRedirect001
