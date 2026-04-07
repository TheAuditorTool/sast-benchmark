//! CWE-327: SHA-1 without salt used for password storage; vulnerable to rainbow table attacks.

// vuln-code-snippet start testcodeCrypto020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let stored_hash = sha1_hash(password.as_bytes()); // vuln-code-snippet target-line testcodeCrypto020
    super::shared::BenchmarkResponse::ok(&format!("stored={}", stored_hash))
}

fn sha1_hash(_data: &[u8]) -> String {
    "da39a3ee5e6b4b0d3255bfef95601890afd80709".to_string()
}
// vuln-code-snippet end testcodeCrypto020
