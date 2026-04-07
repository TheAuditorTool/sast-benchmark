//! CWE-79: All five HTML special characters escaped inline before rendering into HTML.

// vuln-code-snippet start testcodeXss035
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let safe = name.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;").replace('\'', "&#x27;");
    let html = format!("<p>{}</p>", safe); // vuln-code-snippet target-line testcodeXss035

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss035
