//! CWE-79: User input interpolated directly into HTML response.

// vuln-code-snippet start testcodeXss001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let html = format!("<html><body><h1>Hello, {}!</h1></body></html>", name); // vuln-code-snippet target-line testcodeXss001

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss001
