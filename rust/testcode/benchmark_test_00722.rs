pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let config = rcgen_test_ca_config();
    super::shared::BenchmarkResponse::ok(&format!("Test TLS to {} with {}", url, config))
}
fn rcgen_test_ca_config() -> String {
    "rustls(roots=rcgen_test_ca,chain_verified=true)".to_string()
}
