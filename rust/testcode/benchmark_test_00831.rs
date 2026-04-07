pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let ctx = openssl_verify_none();
    super::shared::BenchmarkResponse::ok(&format!("Connected to {} with {}", url, ctx))
}
fn openssl_verify_none() -> String { "openssl(verify=SSL_VERIFY_NONE)".to_string() }
