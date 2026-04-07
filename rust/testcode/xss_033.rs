//! CWE-79: User input placed in a data URI loaded by an iframe, enabling arbitrary HTML execution.

// vuln-code-snippet start testcodeXss033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content = req.param("content");

    let html = format!("<iframe src=\"data:text/html,{}\"></iframe>", content); // vuln-code-snippet target-line testcodeXss033

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss033
