pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("admin_token");
    let cookie = format!("admin={}; Path=/admin; Secure; HttpOnly; SameSite=Strict", token);
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
