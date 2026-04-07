//! CWE-918: Scheme and host regex both validated before fetch.

// vuln-code-snippet start testcodeSsrf036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    if !url.starts_with("https://") {
        return super::shared::BenchmarkResponse::forbidden("Only HTTPS allowed");
    }

    let host = match extract_host(&url) {
        Some(h) => h.to_string(),
        None => return super::shared::BenchmarkResponse::bad_request("Invalid URL"),
    };

    if !is_allowed_domain(&host) {
        return super::shared::BenchmarkResponse::forbidden("Host does not match allowed domain pattern");
    }

    let _resp = simulated_reqwest_get(&url); // vuln-code-snippet target-line testcodeSsrf036

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn extract_host(url: &str) -> Option<&str> {
    let after_scheme = url.strip_prefix("https://")?;
    Some(after_scheme.split('/').next().unwrap_or(""))
}

fn is_allowed_domain(host: &str) -> bool {
    // Simulates: Regex::new(r"^[a-zA-Z0-9.-]+\.example\.com$").unwrap().is_match(host)
    host.ends_with(".example.com")
        && host
            .trim_end_matches(".example.com")
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '.')
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf036
