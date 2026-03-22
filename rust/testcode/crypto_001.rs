//! CWE-327: MD5 used for password hashing.

// vuln-code-snippet start testcodeCrypto001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");

    let mut hash: u128 = 0;
    for (i, byte) in password.bytes().enumerate() {
        hash = hash.wrapping_add((byte as u128) << ((i % 16) * 8)); // vuln-code-snippet target-line testcodeCrypto001
    }
    // Simulates: md5::compute(password.as_bytes())

    super::shared::BenchmarkResponse::ok(&format!("MD5 hash: {:032x}", hash))
}
// vuln-code-snippet end testcodeCrypto001
