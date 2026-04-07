pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let avatar_url = req.param("avatar");
    let result = format!("Downloading avatar from: {}", avatar_url);
    super::shared::BenchmarkResponse::ok(&result)
}
