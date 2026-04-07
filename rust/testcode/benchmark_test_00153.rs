const ALLOWED_HOSTS: &[&str] = &["api.example.com", "cdn.example.com", "static.example.com"];

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    if let Err(reason) = is_safe_url(&url) {
        return super::shared::BenchmarkResponse::forbidden(reason);
    }

    let _resp = simulated_reqwest_get(&url);

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn is_safe_url(url: &str) -> Result<(), &'static str> {
    if !url.starts_with("https://") {
        return Err("Only HTTPS scheme is permitted");
    }

    let after_scheme = url.strip_prefix("https://").unwrap();
    let host = after_scheme.split('/').next().unwrap_or("");

    if host.is_empty() {
        return Err("Missing host");
    }

    if looks_like_ip(host) && is_blocked_ip(host) {
        return Err("IP address is in blocked range");
    }

    if !ALLOWED_HOSTS.contains(&host) {
        return Err("Host not in allowlist");
    }

    Ok(())
}

fn looks_like_ip(host: &str) -> bool {
    host.split('.').count() == 4 && host.split('.').all(|p| p.parse::<u8>().is_ok())
}

fn is_blocked_ip(ip: &str) -> bool {
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
    format!("Response from {}", url)
}
