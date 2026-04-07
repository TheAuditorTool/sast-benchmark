pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    const ALLOWED_DOMAINS: &[&str] = &["api.example.com", "cdn.example.com", "hooks.slack.com"];

    let host = extract_host(&url);
    if !ALLOWED_DOMAINS.contains(&host.as_str()) {
        return super::shared::BenchmarkResponse::bad_request("Domain not in allowlist");
    }

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
