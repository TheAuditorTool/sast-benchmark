//! CWE-79: Double-encoded output. HTML entities encoded before rendering.

// vuln-code-snippet start testcodeXss012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let first_pass = name
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    let double_encoded = first_pass
        .replace('&', "&amp;"); // vuln-code-snippet target-line testcodeXss012

    let html = format!("<html><body><p>{}</p></body></html>", double_encoded);
    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss012
