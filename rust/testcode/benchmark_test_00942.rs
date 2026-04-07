pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _token = req.param("token");
    let cookie = build_static_cookie(&_token);
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn build_static_cookie(_user_input: &str) -> &'static str {
    "session=static-token; Secure; HttpOnly; SameSite=Strict"
}
