//! CWE-79: Askama-style typed template with simulated HTML auto-escaping prevents XSS.

// vuln-code-snippet start testcodeXss046
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

struct PageTemplate<'a> {
    title: &'a str,
}

fn render_template(t: &PageTemplate) -> String {
    let escaped = html_escape(t.title);
    format!("<title>{}</title>", escaped)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let title = req.param("title");

    let tmpl = PageTemplate { title: &title };
    let html = render_template(&tmpl); // vuln-code-snippet target-line testcodeXss046

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss046
