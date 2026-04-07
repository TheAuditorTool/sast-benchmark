pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.param("url");
    if !raw.starts_with("https://cdn.example.com/") {
        return super::shared::BenchmarkResponse::forbidden("Only CDN URLs allowed");
    }
    super::shared::BenchmarkResponse::ok(&format!("Fetching asset: {}", raw))
}
