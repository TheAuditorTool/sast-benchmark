//! CWE-918: User-supplied hostname resolved via DNS then connected to directly.

use std::net::ToSocketAddrs;

// vuln-code-snippet start testcodeSsrf013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let hostname = req.param("host");
    let addr_str = format!("{}:80", hostname);

    let mut addrs = match addr_str.to_socket_addrs() { // vuln-code-snippet target-line testcodeSsrf013
        Ok(a) => a,
        Err(e) => return super::shared::BenchmarkResponse::error(&e.to_string()),
    };

    let resolved = match addrs.next() {
        Some(addr) => addr,
        None => return super::shared::BenchmarkResponse::error("No addresses resolved"),
    };

    // Simulates: std::net::TcpStream::connect(resolved)
    super::shared::BenchmarkResponse::ok(&format!("Connected to {}", resolved))
}
// vuln-code-snippet end testcodeSsrf013
