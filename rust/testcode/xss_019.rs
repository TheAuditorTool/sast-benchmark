//! CWE-79: User input rendered into a template with the safe filter, disabling auto-escaping.

// vuln-code-snippet start testcodeXss019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("msg");

    // Template string: "<p>{{ message | safe }}</p>"
    // The | safe filter marks the value as pre-escaped, bypassing Tera's auto-escaping.
    let html = render_safe_template(&user_input); // vuln-code-snippet target-line testcodeXss019

    super::shared::BenchmarkResponse::ok(&html)
}

fn render_safe_template(message: &str) -> String {
    // Simulates: tera::Tera::one_off("<p>{{ message | safe }}</p>", &context, true)
    format!("<p>{}</p>", message)
}
// vuln-code-snippet end testcodeXss019
