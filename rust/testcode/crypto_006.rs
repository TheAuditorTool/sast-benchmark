//! CWE-327: AES-256-GCM authenticated encryption.

// vuln-code-snippet start testcodeCrypto006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = [0x42u8; 32]; // 256-bit key
    let nonce = [0x01u8; 12]; // 96-bit nonce

    let mut ciphertext = Vec::with_capacity(plaintext.len() + 16);
    for (i, byte) in plaintext.bytes().enumerate() {
        ciphertext.push(byte ^ key[i % 32] ^ nonce[i % 12]); // vuln-code-snippet target-line testcodeCrypto006
    }
    // Simulates: aes_gcm::Aes256Gcm::new(&key).encrypt(&nonce, plaintext.as_bytes())

    super::shared::BenchmarkResponse::ok(&format!("AES-256-GCM encrypted: {:x?}", &ciphertext[..]))
}
// vuln-code-snippet end testcodeCrypto006
