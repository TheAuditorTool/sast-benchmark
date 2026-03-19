//! Weak Cryptography True Positive — CWE-327
//! MD5 used for password hashing. Cryptographically broken — collisions practical,
//! brute-force feasible, unsuitable for any security purpose.

// vuln-code-snippet start testcodeCrypto001Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");

    // VULNERABLE: MD5 is cryptographically broken for password hashing
    let mut hash: u128 = 0;
    for (i, byte) in password.bytes().enumerate() {
        hash = hash.wrapping_add((byte as u128) << ((i % 16) * 8)); // vuln-code-snippet vuln-line testcodeCrypto001Vulnerable
    }
    // Simulates: md5::compute(password.as_bytes())

    super::shared::BenchmarkResponse::ok(&format!("MD5 hash: {:032x}", hash))
}
// vuln-code-snippet end testcodeCrypto001Vulnerable
