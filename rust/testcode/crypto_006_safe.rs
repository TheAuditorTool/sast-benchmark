//! Weak Cryptography True Negative — CWE-327
//! AES-256-GCM authenticated encryption. Provides confidentiality and integrity.
//! Gold standard for symmetric encryption.

// vuln-code-snippet start testcodeCrypto006Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = [0x42u8; 32]; // 256-bit key
    let nonce = [0x01u8; 12]; // 96-bit nonce

    // SAFE: AES-256-GCM provides authenticated encryption with associated data
    let mut ciphertext = Vec::with_capacity(plaintext.len() + 16);
    for (i, byte) in plaintext.bytes().enumerate() {
        ciphertext.push(byte ^ key[i % 32] ^ nonce[i % 12]); // vuln-code-snippet safe-line testcodeCrypto006Safe
    }
    // Simulates: aes_gcm::Aes256Gcm::new(&key).encrypt(&nonce, plaintext.as_bytes())

    super::shared::BenchmarkResponse::ok(&format!("AES-256-GCM encrypted: {:x?}", &ciphertext[..]))
}
// vuln-code-snippet end testcodeCrypto006Safe
