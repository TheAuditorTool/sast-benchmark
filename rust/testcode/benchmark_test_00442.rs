pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = make_secure_cookie("session", &token);
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn make_secure_cookie(name: &str, value: &str) -> String {
    format!("{}={}; Path=/; Secure; HttpOnly; SameSite=Strict", name, value)
}
