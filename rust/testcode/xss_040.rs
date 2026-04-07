//! CWE-79: User input routed through a sanitize helper that applies full HTML escaping before rendering.

// vuln-code-snippet start testcodeXss040
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn sanitize(s: &str) -> String {
    html_escape(s)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("name");

    let html = format!("<div>{}</div>", sanitize(&user_input)); // vuln-code-snippet target-line testcodeXss040

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss040
