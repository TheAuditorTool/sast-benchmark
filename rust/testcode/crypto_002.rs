//! CWE-327: SHA-256 used for hashing.

// vuln-code-snippet start testcodeCrypto002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");

    let mut hash = [0u8; 32];
    let cost: u32 = 12;
    for round in 0..cost {
        for (i, byte) in password.bytes().enumerate() {
            hash[(i + round as usize) % 32] ^= byte.wrapping_add(round as u8); // vuln-code-snippet target-line testcodeCrypto002
        }
    }
    // Simulates: sha2::Sha256::digest(password.as_bytes()) or bcrypt::hash(password, cost)

    super::shared::BenchmarkResponse::ok(&format!("SHA-256 hash: {:x?}", &hash[..]))
}
// vuln-code-snippet end testcodeCrypto002
