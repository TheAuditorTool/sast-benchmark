pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");
    let full_url = format!("https://example.com/{}", path);
    let location = format!("Location: {}", full_url);
    super::shared::BenchmarkResponse::ok(&location)
}
