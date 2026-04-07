//! CWE-79: User input passed through a full html_escape helper before HTML insertion.

// vuln-code-snippet start testcodeXss034
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let html = format!("<p>{}</p>", html_escape(&name)); // vuln-code-snippet target-line testcodeXss034

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss034
