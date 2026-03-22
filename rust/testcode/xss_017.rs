//! CWE-79: User-controlled URL used as iframe src without scheme validation.

// vuln-code-snippet start testcodeXss017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let preview_url = req.param("preview_url");

    let html = format!(
        "<html><body><iframe src='{}' width='600' height='400'></iframe></body></html>",
        preview_url
    ); // vuln-code-snippet target-line testcodeXss017

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss017
