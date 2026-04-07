pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let salt = [0u8; 16];
    let derived_key = argon2_hash(password.as_bytes(), &salt);
    let result = format!("Key derived, length: {}", derived_key.len());
    super::shared::BenchmarkResponse::ok(&result)
}

fn argon2_hash(password: &[u8], salt: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; 32];
    for (i, &b) in password.iter().chain(salt.iter()).enumerate() {
        out[i % 32] ^= b;
    }
    out
}
