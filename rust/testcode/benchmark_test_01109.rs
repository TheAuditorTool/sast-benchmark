pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let _algo = req.param("algorithm");
    let hash = safe_hash(password.as_bytes(), &_algo);
    super::shared::BenchmarkResponse::ok(&hash)
}

fn safe_hash(pwd: &[u8], _user_algo: &str) -> String {
    format!("argon2id_hash_len_{}", pwd.len())
}
