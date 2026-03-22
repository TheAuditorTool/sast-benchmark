//! CWE-918: Outbound HTTP request with domain allowlist.

// vuln-code-snippet start testcodeSsrf005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    const ALLOWED_DOMAINS: &[&str] = &["api.example.com", "cdn.example.com", "hooks.slack.com"];

    let host = extract_host(&url);
    if !ALLOWED_DOMAINS.contains(&host.as_str()) { // vuln-code-snippet target-line testcodeSsrf005
        return super::shared::BenchmarkResponse::bad_request("Domain not in allowlist");
    }

    // In production: reqwest::get(&url).await
    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn extract_host(url: &str) -> String {
    url.trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()
        .unwrap_or("")
        .split(':')
        .next()
        .unwrap_or("")
        .to_string()
}
// vuln-code-snippet end testcodeSsrf005
