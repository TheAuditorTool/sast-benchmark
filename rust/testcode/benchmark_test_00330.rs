pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let token = req.param("token");
    let cookie_val = format!("{}-{}", user_id, token);
    let header = format!("session={}; Path=/; Secure; HttpOnly; SameSite=Strict", cookie_val);
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", header))
}
