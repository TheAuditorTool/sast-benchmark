//! CWE-327: scrypt with high cost parameters used; memory-hard function appropriate for passwords.

// vuln-code-snippet start testcodeCrypto036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let salt = generate_random_salt();
    let hash = scrypt_hash(password.as_bytes(), &salt); // vuln-code-snippet target-line testcodeCrypto036
    super::shared::BenchmarkResponse::ok(&format!("hash={}", hash))
}

fn generate_random_salt() -> [u8; 16] { [0u8; 16] }
fn scrypt_hash(_pwd: &[u8], _salt: &[u8]) -> String { "SCRYPT_HASH".to_string() }
// vuln-code-snippet end testcodeCrypto036
