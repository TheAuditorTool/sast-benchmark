//! CWE-798: Encryption key derived from user password via argon2 at runtime.

// vuln-code-snippet start testcodeHardcodedcreds020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let salt = [0u8; 16]; // In production, use random salt
    // Simulates: argon2::hash_encoded(password.as_bytes(), &salt, &config)
    let derived_key = argon2_hash(password.as_bytes(), &salt); // vuln-code-snippet target-line testcodeHardcodedcreds020
    let result = format!("Key derived, length: {}", derived_key.len());
    super::shared::BenchmarkResponse::ok(&result)
}

fn argon2_hash(password: &[u8], salt: &[u8]) -> Vec<u8> {
    // Simulates: argon2::hash_encoded(password, salt, &Config::default())
    let mut out = vec![0u8; 32];
    for (i, &b) in password.iter().chain(salt.iter()).enumerate() {
        out[i % 32] ^= b;
    }
    out
}
// vuln-code-snippet end testcodeHardcodedcreds020
