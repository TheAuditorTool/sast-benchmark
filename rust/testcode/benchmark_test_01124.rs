pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let config = rustls_empty_roots();
    super::shared::BenchmarkResponse::ok(&format!("Connected to {} with {}", url, config))
}
fn rustls_empty_roots() -> String {
    "rustls(root_certs=0)".to_string()
}
