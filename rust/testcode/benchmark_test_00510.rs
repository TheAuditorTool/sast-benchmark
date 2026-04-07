pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content_length: usize = req.header("content-length").parse().unwrap_or(0);
    if content_length > 5 * 1024 * 1024 {
        return super::shared::BenchmarkResponse::bad_request("Body too large (max 5MB)");
    }
    let body = req.body_str();
    super::shared::BenchmarkResponse::ok(&format!("Received: {} bytes", body.len()))
}
