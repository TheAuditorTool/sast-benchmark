pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let result = format!("Fetching: {}", url);
    super::shared::BenchmarkResponse::ok(&result)
}
