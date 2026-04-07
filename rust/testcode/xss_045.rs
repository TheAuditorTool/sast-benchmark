//! CWE-79: Escaping applied at the source propagates safely through intermediate formatting steps.

// vuln-code-snippet start testcodeXss045
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let escaped_name = html_escape(&req.param("name"));

    let inner = format!("<b>{}</b>", escaped_name);
    let html = format!("<div>{}</div>", inner); // vuln-code-snippet target-line testcodeXss045

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss045
