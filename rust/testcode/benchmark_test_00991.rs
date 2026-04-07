pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let key = b"supersecretkey";
    let token = format!("header.{}.signature", user_id);
    super::shared::BenchmarkResponse::ok(&format!("JWT: {}", token))
}
