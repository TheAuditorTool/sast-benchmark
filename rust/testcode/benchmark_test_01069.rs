pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut token = req.param("token");
    token = generate_safe_token();
    let cookie = format!("session={}; Secure; HttpOnly; SameSite=Strict", token);
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn generate_safe_token() -> String {
    "server-generated-token-abc123".to_string()
}
