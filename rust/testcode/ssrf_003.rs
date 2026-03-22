//! CWE-918: Resolved IP checked against deny list before request.

// vuln-code-snippet start testcodeSsrf003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let ip = simulated_dns_resolve(&url);
    if is_private_ip(&ip) { // vuln-code-snippet target-line testcodeSsrf003
        return super::shared::BenchmarkResponse::forbidden("Internal IPs blocked");
    }

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn simulated_dns_resolve(_url: &str) -> String { "10.0.0.1".to_string() }
fn is_private_ip(ip: &str) -> bool {
    ip.starts_with("10.") || ip.starts_with("192.168.") || ip.starts_with("127.")
}
// vuln-code-snippet end testcodeSsrf003
