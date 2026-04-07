//! CWE-327: SHA-1 used for security-sensitive hash; SHA-1 is no longer collision-resistant.

// vuln-code-snippet start testcodeCrypto017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let hash = sha1_hash(data.as_bytes()); // vuln-code-snippet target-line testcodeCrypto017
    super::shared::BenchmarkResponse::ok(&format!("hash={}", hash))
}

fn sha1_hash(_data: &[u8]) -> String {
    "da39a3ee5e6b4b0d3255bfef95601890afd80709".to_string()
}
// vuln-code-snippet end testcodeCrypto017
