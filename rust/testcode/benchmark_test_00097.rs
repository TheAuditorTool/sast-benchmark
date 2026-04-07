pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let location = format!("Location: {}", url);
    super::shared::BenchmarkResponse::ok(&location)
}
