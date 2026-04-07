pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let ip = dns_resolve(&url);
    if is_internal(&ip) {
        return super::shared::BenchmarkResponse::forbidden("Resolved to internal IP");
    }

    super::shared::BenchmarkResponse::ok(&format!("Fetched (resolved {}): {}", ip, url))
}

fn dns_resolve(_url: &str) -> String { "203.0.113.1".to_string() }
fn is_internal(ip: &str) -> bool {
    ip.starts_with("10.") || ip.starts_with("172.16.") || ip.starts_with("192.168.") || ip == "127.0.0.1"
}
