pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let connector = native_tls_default();
    super::shared::BenchmarkResponse::ok(&format!("Connected to {} via {}", url, connector))
}
fn native_tls_default() -> String { "native_tls(verify=default)".to_string() }
