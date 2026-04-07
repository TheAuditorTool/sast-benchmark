//! CWE-327: SHA-256 applied directly to password without salt; fast hash unsuitable for passwords.

// vuln-code-snippet start testcodeCrypto026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let hash = sha256_hash(password.as_bytes()); // vuln-code-snippet target-line testcodeCrypto026
    super::shared::BenchmarkResponse::ok(&format!("pwd_hash={}", hash))
}

fn sha256_hash(_data: &[u8]) -> String {
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string()
}
// vuln-code-snippet end testcodeCrypto026
