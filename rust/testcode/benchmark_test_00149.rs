pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_url = req.param("url");
    let header = format!("Location: {}", user_url);
    super::shared::BenchmarkResponse::ok(&format!("302 Redirect\n{}", header))
}
