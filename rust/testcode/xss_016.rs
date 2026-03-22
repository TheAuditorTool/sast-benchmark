//! CWE-79: User input placed inside HTML comment without escaping.

// vuln-code-snippet start testcodeXss016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let query = req.param("q");

    let html = format!(
        "<html><!-- search query: {} --><body><p>Results for your query</p></body></html>",
        query
    ); // vuln-code-snippet target-line testcodeXss016

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss016
