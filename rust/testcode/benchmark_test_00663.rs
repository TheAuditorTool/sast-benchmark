pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_path = req.param("path");
    let redirect = format!("/app/{}", user_path);
    super::shared::BenchmarkResponse::ok(&format!("Location: {}", redirect))
}
