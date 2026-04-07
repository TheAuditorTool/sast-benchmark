//! CWE-327: PBKDF2 configured with 1 iteration; insufficient work factor makes it equivalent to a fast hash.

// vuln-code-snippet start testcodeCrypto024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let salt = b"fixedsalt";
    let hash = pbkdf2(password.as_bytes(), salt, 1); // vuln-code-snippet target-line testcodeCrypto024
    super::shared::BenchmarkResponse::ok(&format!("hash={}", hash))
}

fn pbkdf2(_pwd: &[u8], _salt: &[u8], iterations: u32) -> String {
    format!("pbkdf2_{}iter", iterations)
}
// vuln-code-snippet end testcodeCrypto024
