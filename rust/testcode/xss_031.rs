//! CWE-79: User-controlled URL injected into a meta refresh tag enabling open redirect and XSS.

// vuln-code-snippet start testcodeXss031
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let html = format!("<meta http-equiv=\"refresh\" content=\"0;url={}\">", url); // vuln-code-snippet target-line testcodeXss031

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss031
