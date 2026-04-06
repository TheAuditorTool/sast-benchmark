//! CWE-601: Redirect domain checked against allowlist loaded from configuration.

// vuln-code-snippet start testcodeRedirect016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let target = req.param("url");
    let allowed_domains = get_allowed_domains();
    let host = extract_host(&target);
    if allowed_domains.contains(&host.as_str()) { // vuln-code-snippet target-line testcodeRedirect016
        super::shared::BenchmarkResponse::ok(&format!("Location: {}", target))
    } else {
        super::shared::BenchmarkResponse::bad_request("Domain not allowed")
    }
}
fn get_allowed_domains() -> Vec<&'static str> { vec!["example.com", "app.example.com"] }
fn extract_host(url: &str) -> String {
    url.split("://").nth(1).and_then(|s| s.split('/').next()).unwrap_or("").to_string()
}
// vuln-code-snippet end testcodeRedirect016
