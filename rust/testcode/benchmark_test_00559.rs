pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let salt = b"fixedsalt";
    let hash = pbkdf2(password.as_bytes(), salt, 1);
    super::shared::BenchmarkResponse::ok(&format!("hash={}", hash))
}

fn pbkdf2(_pwd: &[u8], _salt: &[u8], iterations: u32) -> String {
    format!("pbkdf2_{}iter", iterations)
}
