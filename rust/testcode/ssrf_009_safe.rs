//! SSRF True Negative — CWE-918
//! Pre-resolved DNS check. DNS is resolved first, then the IP is validated
//! against a deny list before making the actual request.

// vuln-code-snippet start testcodeSsrf009Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    // SAFE: Resolve DNS first, validate IP before fetch
    let ip = dns_resolve(&url);
    if is_internal(&ip) { // vuln-code-snippet safe-line testcodeSsrf009Safe
        return super::shared::BenchmarkResponse::forbidden("Resolved to internal IP");
    }

    super::shared::BenchmarkResponse::ok(&format!("Fetched (resolved {}): {}", ip, url))
}

fn dns_resolve(_url: &str) -> String { "203.0.113.1".to_string() }
fn is_internal(ip: &str) -> bool {
    ip.starts_with("10.") || ip.starts_with("172.16.") || ip.starts_with("192.168.") || ip == "127.0.0.1"
}
// vuln-code-snippet end testcodeSsrf009Safe
