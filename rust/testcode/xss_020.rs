//! CWE-79: User input rendered via a typed Askama template with default HTML auto-escaping.

// vuln-code-snippet start testcodeXss020
struct GreetTemplate<'a> {
    name: &'a str,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let tmpl = GreetTemplate { name: &name };
    let html = render_template(&tmpl); // vuln-code-snippet target-line testcodeXss020

    super::shared::BenchmarkResponse::ok(&html)
}

fn render_template(tmpl: &GreetTemplate) -> String {
    // Simulates: askama::Template::render(&tmpl)
    // Askama escapes HTML entities in &str fields by default (no | safe equivalent).
    let escaped = tmpl.name
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;");
    format!("<p>Hello, {}!</p>", escaped)
}
// vuln-code-snippet end testcodeXss020
