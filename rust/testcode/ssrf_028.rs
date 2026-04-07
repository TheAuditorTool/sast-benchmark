//! CWE-918: Private IP blocklist — resolved IP checked before fetch.

// vuln-code-snippet start testcodeSsrf028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let host = match extract_host(&url) {
        Some(h) => h.to_string(),
        None => return super::shared::BenchmarkResponse::bad_request("Invalid URL"),
    };

    let resolved_ip = simulated_dns_resolve(&host);

    if is_internal_ip(&resolved_ip) {
        return super::shared::BenchmarkResponse::forbidden("Resolved to private IP");
    }

    let _resp = simulated_reqwest_get(&url); // vuln-code-snippet target-line testcodeSsrf028

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn extract_host(url: &str) -> Option<&str> {
    let after_scheme = url.strip_prefix("https://").or_else(|| url.strip_prefix("http://"))?;
    Some(after_scheme.split('/').next().unwrap_or(""))
}

fn simulated_dns_resolve(_host: &str) -> String {
    // In production: use std::net::ToSocketAddrs to resolve
    "203.0.113.42".to_string()
}

fn is_internal_ip(ip: &str) -> bool {
    ip.starts_with("127.")
        || ip.starts_with("10.")
        || ip.starts_with("192.168.")
        || ip.starts_with("169.254.")
        || {
            let parts: Vec<&str> = ip.split('.').collect();
            if parts.len() == 4 {
                if let Ok(second) = parts[1].parse::<u8>() {
                    parts[0] == "172" && (16..=31).contains(&second)
                } else {
                    false
                }
            } else {
                false
            }
        }
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf028
