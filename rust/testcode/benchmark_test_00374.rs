pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let value = req.param("session_id");

    let cookie = rocket_add_private("session", &value);

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn rocket_add_private(name: &str, value: &str) -> String {
    format!("{}=encrypted_{}; Path=/; Secure; HttpOnly", name, value)
}
