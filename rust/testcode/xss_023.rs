//! CWE-79: User input placed inside a javascript: URI in an anchor href.

// vuln-code-snippet start testcodeXss023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");

    let html = format!("<a href=\"javascript:{}\">Click</a>", action); // vuln-code-snippet target-line testcodeXss023

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss023
