pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let connector = native_tls_danger(true);
    super::shared::BenchmarkResponse::ok(&format!("Connected to {} via {}", url, connector))
}
fn native_tls_danger(accept_invalid: bool) -> String {
    format!("native_tls(danger={})", accept_invalid)
}
