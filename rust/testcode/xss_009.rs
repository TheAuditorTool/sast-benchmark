//! CWE-79: Template engine with auto-escaping (simulated Askama/Tera).

// vuln-code-snippet start testcodeXss009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let auto_escaped = name
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;"); // vuln-code-snippet target-line testcodeXss009
    // Simulates: askama::Template with auto-escape = "html"

    let html = format!("<html><body><p>Welcome, {}!</p></body></html>", auto_escaped);
    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss009
