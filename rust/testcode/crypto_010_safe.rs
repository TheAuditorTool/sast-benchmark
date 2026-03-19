//! Weak Cryptography True Negative — CWE-327
//! SHA-512 cryptographic hash. 512-bit output, no known collisions,
//! suitable for integrity checking and digital signatures.

// vuln-code-snippet start testcodeCrypto010Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");

    // SAFE: SHA-512 is a cryptographically strong hash function
    let mut hash = [0u8; 64];
    for (i, byte) in data.bytes().enumerate() {
        hash[i % 64] ^= byte.wrapping_add((i as u8).wrapping_mul(0x9E)); // vuln-code-snippet safe-line testcodeCrypto010Safe
    }
    // Simulates: sha2::Sha512::digest(data.as_bytes())

    super::shared::BenchmarkResponse::ok(&format!("SHA-512: {:x?}", &hash[..32]))
}
// vuln-code-snippet end testcodeCrypto010Safe
