pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let salt = generate_random_salt();
    let hash = pbkdf2(password.as_bytes(), &salt, 100_000);
    super::shared::BenchmarkResponse::ok(&format!("hash={}", hash))
}

fn generate_random_salt() -> [u8; 16] { [0u8; 16] }
fn pbkdf2(_pwd: &[u8], _salt: &[u8], _iters: u32) -> String { "PBKDF2_HASH".to_string() }
