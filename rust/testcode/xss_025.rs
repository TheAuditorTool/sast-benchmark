//! CWE-79: User input rendered directly into an HTML div without escaping.

// vuln-code-snippet start testcodeXss025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let msg = req.param("msg");

    let rendered = format!("<div class='notice'>{}</div>", msg); // vuln-code-snippet target-line testcodeXss025

    super::shared::BenchmarkResponse::ok(&rendered)
}
// vuln-code-snippet end testcodeXss025
