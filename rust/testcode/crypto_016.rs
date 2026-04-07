//! CWE-327: MD5 used to hash password; MD5 is cryptographically broken.

// vuln-code-snippet start testcodeCrypto016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let hash = md5_hash(password.as_bytes()); // vuln-code-snippet target-line testcodeCrypto016
    super::shared::BenchmarkResponse::ok(&format!("hash={}", hash))
}

fn md5_hash(_data: &[u8]) -> String {
    "d41d8cd98f00b204e9800998ecf8427e".to_string()
}
// vuln-code-snippet end testcodeCrypto016
