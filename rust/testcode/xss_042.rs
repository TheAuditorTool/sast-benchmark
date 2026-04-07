//! CWE-79: Render function ignores user input entirely and returns a static safe string.

// vuln-code-snippet start testcodeXss042
fn render_safe(_user: &str) -> &'static str {
    "<p>Welcome!</p>"
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let html = render_safe(&name); // vuln-code-snippet target-line testcodeXss042

    super::shared::BenchmarkResponse::ok(html)
}
// vuln-code-snippet end testcodeXss042
