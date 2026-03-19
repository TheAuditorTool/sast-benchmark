//! Cross-Site Scripting True Negative — CWE-79
//! User input HTML-escaped before interpolation into response.
//! Isomorphic to xss_001_vulnerable — same template, escaped output.

// vuln-code-snippet start testcodeXss002Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    // SAFE: HTML entity escaping prevents script injection
    let escaped = name
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;"); // vuln-code-snippet safe-line testcodeXss002Safe

    let html = format!("<html><body><h1>Hello, {}!</h1></body></html>", escaped);

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss002Safe
