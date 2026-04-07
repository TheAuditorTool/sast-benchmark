pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let value = req.param("data");

    let encrypted_cookie = encrypted_jar_add("session", &value);

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", encrypted_cookie))
}

fn encrypted_jar_add(name: &str, value: &str) -> String {
    let encrypted = format!("enc_{}", value);
    format!("{}={}; Path=/; Secure; HttpOnly", name, encrypted)
}
