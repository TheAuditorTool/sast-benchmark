//! CWE-79: HTML escaping applied when storing input in a struct field, preventing XSS at render time.

// vuln-code-snippet start testcodeXss036
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

struct Profile {
    name: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let profile = Profile {
        name: html_escape(&req.param("name")),
    };

    let html = format!("<h2>{}</h2>", profile.name); // vuln-code-snippet target-line testcodeXss036

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss036
