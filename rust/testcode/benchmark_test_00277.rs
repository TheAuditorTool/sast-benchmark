use std::net::ToSocketAddrs;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let hostname = req.param("host");
    let addr_str = format!("{}:80", hostname);

    let mut addrs = match addr_str.to_socket_addrs() {
        Ok(a) => a,
        Err(e) => return super::shared::BenchmarkResponse::error(&e.to_string()),
    };

    let resolved = match addrs.next() {
        Some(addr) => addr,
        None => return super::shared::BenchmarkResponse::error("No addresses resolved"),
    };

    super::shared::BenchmarkResponse::ok(&format!("Connected to {}", resolved))
}
