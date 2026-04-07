pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let config = TlsConfig { verify_certs: true, verify_hostname: true };
    let result = make_request(&url, &config);
    super::shared::BenchmarkResponse::ok(&result)
}

struct TlsConfig { verify_certs: bool, verify_hostname: bool }
fn make_request(url: &str, cfg: &TlsConfig) -> String {
    format!("GET {} verified={}", url, cfg.verify_certs && cfg.verify_hostname)
}
