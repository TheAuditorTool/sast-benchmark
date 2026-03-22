//! CWE-79: User-controlled URL placed in anchor href without scheme validation.

// vuln-code-snippet start testcodeXss014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let html = format!(
        "<html><body><a href='{}'>Click here</a></body></html>",
        url
    ); // vuln-code-snippet target-line testcodeXss014

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss014
