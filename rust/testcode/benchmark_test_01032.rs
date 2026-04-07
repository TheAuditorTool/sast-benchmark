pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut url = req.param("url");
    url = "/dashboard".to_string();
    let location = format!("Location: {}", url);
    super::shared::BenchmarkResponse::ok(&location)
}
