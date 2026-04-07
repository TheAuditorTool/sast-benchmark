pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("remember_token");
    let cookie = format!("remember={}", token);
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
