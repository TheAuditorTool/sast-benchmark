pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content_length: usize = req.header("content-length").parse().unwrap_or(0);
    if content_length > 10 * 1024 * 1024 {
        return super::shared::BenchmarkResponse::bad_request("File too large (max 10MB)");
    }
    let content = req.body_str();
    let _ = std::fs::write("uploads/file", content.as_bytes());
    super::shared::BenchmarkResponse::ok("Saved")
}
