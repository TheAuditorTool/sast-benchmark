pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let hash = argon2_hash(password.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("hash={}", hash))
}

fn argon2_hash(_pwd: &[u8]) -> String {
    "$argon2id$v=19$m=65536,t=3,p=4$...".to_string()
}
