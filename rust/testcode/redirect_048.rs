//! CWE-601: Redirect helper discards user-supplied URL and returns fixed safe destination.

// vuln-code-snippet start testcodeRedirect048
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let location = safe_location(&url); // vuln-code-snippet target-line testcodeRedirect048
    super::shared::BenchmarkResponse::ok(location)
}

fn safe_location(_user_url: &str) -> &'static str {
    "Location: /home"
}
// vuln-code-snippet end testcodeRedirect048
