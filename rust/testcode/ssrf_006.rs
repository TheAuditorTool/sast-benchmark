//! CWE-918: Outbound HTTP request with private IP rejection.

// vuln-code-snippet start testcodeSsrf006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let host = url.trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()
        .unwrap_or("")
        .split(':')
        .next()
        .unwrap_or("");

    if is_private_or_loopback(host) { // vuln-code-snippet target-line testcodeSsrf006
        return super::shared::BenchmarkResponse::bad_request("Internal addresses blocked");
    }

    // In production: reqwest::get(&url).await
    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn is_private_or_loopback(host: &str) -> bool {
    host == "localhost"
        || host.starts_with("127.")
        || host.starts_with("10.")
        || host.starts_with("192.168.")
        || host.starts_with("172.16.") || host.starts_with("172.17.")
        || host.starts_with("172.18.") || host.starts_with("172.19.")
        || host.starts_with("172.20.") || host.starts_with("172.21.")
        || host.starts_with("172.22.") || host.starts_with("172.23.")
        || host.starts_with("172.24.") || host.starts_with("172.25.")
        || host.starts_with("172.26.") || host.starts_with("172.27.")
        || host.starts_with("172.28.") || host.starts_with("172.29.")
        || host.starts_with("172.30.") || host.starts_with("172.31.")
        || host.starts_with("169.254.")
        || host == "::1"
        || host == "[::1]"
        || host.starts_with("0.")
        || host == "0.0.0.0"
}
// vuln-code-snippet end testcodeSsrf006
