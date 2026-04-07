//! CWE-327: Password hashed with bcrypt (cost=12); adaptive work factor prevents brute-force.

// vuln-code-snippet start testcodeCrypto030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let hash = bcrypt_hash(password.as_bytes(), 12); // vuln-code-snippet target-line testcodeCrypto030
    super::shared::BenchmarkResponse::ok(&format!("hash={}", hash))
}

fn bcrypt_hash(_pwd: &[u8], _cost: u32) -> String {
    "$2b$12$...".to_string()
}
// vuln-code-snippet end testcodeCrypto030
