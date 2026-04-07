pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    let cookie = tower_cookies_secure("session", &token);

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn tower_cookies_secure(name: &str, value: &str) -> String {
    format!("{}={}; Path=/; Secure; HttpOnly; SameSite=Lax", name, value)
}
