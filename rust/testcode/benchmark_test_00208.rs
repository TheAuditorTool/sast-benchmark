pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = format!("session={}; SameSite=Lax", token);
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
