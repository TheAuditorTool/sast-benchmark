pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let salt = generate_random_salt();
    let hash = scrypt_hash(password.as_bytes(), &salt);
    super::shared::BenchmarkResponse::ok(&format!("hash={}", hash))
}

fn generate_random_salt() -> [u8; 16] { [0u8; 16] }
fn scrypt_hash(_pwd: &[u8], _salt: &[u8]) -> String { "SCRYPT_HASH".to_string() }
