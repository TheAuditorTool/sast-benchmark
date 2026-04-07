pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let config = mtls_config();
    super::shared::BenchmarkResponse::ok(&format!("mTLS connection to {} with {}", url, config))
}
fn mtls_config() -> String {
    "rustls(client_cert=client.pem,server_verify=true)".to_string()
}
