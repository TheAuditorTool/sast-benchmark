pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    let max_age = 365 * 24 * 60 * 60;
    let cookie = format!("session={}; Path=/; Max-Age={}", token, max_age);

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
