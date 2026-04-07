pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let config = rustls_webpki_roots();
    super::shared::BenchmarkResponse::ok(&format!("Connected to {} with {}", url, config))
}
fn rustls_webpki_roots() -> String { "rustls(roots=webpki_roots::TLS_SERVER_ROOTS)".to_string() }
