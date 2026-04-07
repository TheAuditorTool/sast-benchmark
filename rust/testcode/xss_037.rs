//! CWE-79: Dead-code branch — constant condition always selects the escaped path.

// vuln-code-snippet start testcodeXss037
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    // 7 * 6 == 42 > 40 is always true; the else branch is dead code.
    let display = if 7 * 6 > 40 { html_escape(&name) } else { name.clone() };
    let html = format!("<p>{}</p>", display); // vuln-code-snippet target-line testcodeXss037

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss037
