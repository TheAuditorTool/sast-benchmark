//! CWE-918: Domain allowlist — only pre-approved hosts are reachable.

// vuln-code-snippet start testcodeSsrf027
const ALLOWED_HOSTS: &[&str] = &["api.example.com", "cdn.example.com"];

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let host = match extract_host(&url) {
        Some(h) => h,
        None => return super::shared::BenchmarkResponse::bad_request("Invalid URL"),
    };

    if !ALLOWED_HOSTS.contains(&host) {
        return super::shared::BenchmarkResponse::forbidden("Host not in allowlist");
    }

    let _resp = simulated_reqwest_get(&url); // vuln-code-snippet target-line testcodeSsrf027

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn extract_host(url: &str) -> Option<&str> {
    // In production: url::Url::parse(url).ok().and_then(|u| u.host_str())
    let after_scheme = url.strip_prefix("https://").or_else(|| url.strip_prefix("http://"))?;
    Some(after_scheme.split('/').next().unwrap_or(""))
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf027
