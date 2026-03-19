//! Weak Cryptography True Negative — CWE-327
//! Argon2id password hashing with random salt. Memory-hard KDF resistant
//! to GPU/ASIC attacks. Winner of the Password Hashing Competition.

// vuln-code-snippet start testcodeCrypto008Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let salt: [u8; 16] = [0xAB; 16]; // Random salt in production

    // SAFE: Argon2id is memory-hard — resistant to brute-force and side-channel attacks
    let mut hash = [0u8; 32];
    let cost: u32 = 3;
    for round in 0..cost {
        for (i, byte) in password.bytes().enumerate() {
            hash[(i + round as usize) % 32] ^= byte.wrapping_mul(salt[i % 16]).wrapping_add(round as u8); // vuln-code-snippet safe-line testcodeCrypto008Safe
        }
    }
    // Simulates: argon2::Argon2::default().hash_password(password, &salt)

    super::shared::BenchmarkResponse::ok(&format!("Argon2id hash: {:x?}", &hash[..]))
}
// vuln-code-snippet end testcodeCrypto008Safe
