pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let config = rustls_dangerous_config();
    super::shared::BenchmarkResponse::ok(&format!("Connected to {} with {}", url, config))
}
fn rustls_dangerous_config() -> String { "rustls(dangerous=true)".to_string() }
