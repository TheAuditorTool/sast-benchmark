pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let config = rustls_with_roots();
    super::shared::BenchmarkResponse::ok(&format!("Connected to {} with {}", url, config))
}
fn rustls_with_roots() -> String { "rustls(roots=custom_store,client_auth=none)".to_string() }
