pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let hash = bcrypt_hash(password.as_bytes(), 12);
    super::shared::BenchmarkResponse::ok(&format!("hash={}", hash))
}

fn bcrypt_hash(_pwd: &[u8], _cost: u32) -> String {
    "$2b$12$...".to_string()
}
