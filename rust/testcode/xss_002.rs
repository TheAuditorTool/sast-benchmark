//! CWE-79: User input HTML-escaped before interpolation into response.

// vuln-code-snippet start testcodeXss002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let escaped = name
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;"); // vuln-code-snippet target-line testcodeXss002

    let html = format!("<html><body><h1>Hello, {}!</h1></body></html>", escaped);

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss002
