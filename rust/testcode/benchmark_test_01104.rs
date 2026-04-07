pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let channel = tonic_no_ca(&endpoint);
    super::shared::BenchmarkResponse::ok(&format!("gRPC channel: {}", channel))
}
fn tonic_no_ca(endpoint: &str) -> String { format!("tonic(endpoint={},ca=none)", endpoint) }
