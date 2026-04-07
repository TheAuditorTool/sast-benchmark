pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");

    let remember_value = format!("remember_{}", user_id);
    let cookie = format!("remember_me={}", remember_value);

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
