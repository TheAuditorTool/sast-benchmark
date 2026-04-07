pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    let cookie = cookie_build_all_flags("session", &token);

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn cookie_build_all_flags(name: &str, value: &str) -> String {
    format!("{}={}; Path=/; Secure; HttpOnly; SameSite=Strict", name, value)
}
