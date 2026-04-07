pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");

    let jwt = format!("header.{}.signature", user_id);
    let cookie = format!("jwt={}; Path=/; Secure; HttpOnly; SameSite=Strict", jwt);

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
