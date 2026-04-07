pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.body_str();
    super::shared::BenchmarkResponse::ok(&format!("Received {} bytes", body.len()))
}
