//! CWE-601: User-supplied URL unconditionally overwritten with safe constant before redirect.

// vuln-code-snippet start testcodeRedirect047
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut url = req.param("url");
    url = "/dashboard".to_string();
    let location = format!("Location: {}", url); // vuln-code-snippet target-line testcodeRedirect047
    super::shared::BenchmarkResponse::ok(&location)
}
// vuln-code-snippet end testcodeRedirect047
