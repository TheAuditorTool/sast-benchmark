pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let salt = generate_random_salt();
    let hash = sha256_with_salt(data.as_bytes(), &salt);
    super::shared::BenchmarkResponse::ok(&format!("integrity={}", hash))
}

fn generate_random_salt() -> [u8; 16] { [0u8; 16] }
fn sha256_with_salt(_data: &[u8], _salt: &[u8]) -> String { "SHA256_SALTED".to_string() }
