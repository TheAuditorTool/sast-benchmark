//! CWE-601: User-supplied URL stored in struct and forwarded as redirect target without validation.

// vuln-code-snippet start testcodeRedirect026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let redir = RedirectTarget { url: req.param("url") };
    let location = build_location(&redir.url); // vuln-code-snippet target-line testcodeRedirect026
    super::shared::BenchmarkResponse::ok(&location)
}

struct RedirectTarget { url: String }

fn build_location(url: &str) -> String {
    format!("Location: {}", url)
}
// vuln-code-snippet end testcodeRedirect026
