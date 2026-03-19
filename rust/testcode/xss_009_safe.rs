//! Cross-Site Scripting True Negative — CWE-79
//! Template engine with auto-escaping enabled (simulated Askama/Tera).
//! All interpolated values are HTML-escaped by default.

// vuln-code-snippet start testcodeXss009Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    // SAFE: Simulated auto-escaping template engine (Askama/Tera default)
    let auto_escaped = name
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;"); // vuln-code-snippet safe-line testcodeXss009Safe
    // Simulates: askama::Template with auto-escape = "html"

    let html = format!("<html><body><p>Welcome, {}!</p></body></html>", auto_escaped);
    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss009Safe
