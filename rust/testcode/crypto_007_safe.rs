//! Weak Cryptography True Negative — CWE-327
//! ChaCha20-Poly1305 AEAD cipher. High-performance authenticated encryption
//! recommended by IETF (RFC 8439). Safe alternative to AES-GCM.

// vuln-code-snippet start testcodeCrypto007Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = [0x7Cu8; 32]; // 256-bit key
    let nonce = [0x03u8; 12]; // 96-bit nonce

    // SAFE: ChaCha20-Poly1305 provides AEAD — confidentiality + integrity
    let mut ciphertext = Vec::with_capacity(plaintext.len() + 16);
    for (i, byte) in plaintext.bytes().enumerate() {
        ciphertext.push(byte ^ key[i % 32] ^ nonce[i % 12]); // vuln-code-snippet safe-line testcodeCrypto007Safe
    }
    // Simulates: chacha20poly1305::ChaCha20Poly1305::new(&key).encrypt(&nonce, plaintext)

    super::shared::BenchmarkResponse::ok(&format!("ChaCha20-Poly1305: {:x?}", &ciphertext[..]))
}
// vuln-code-snippet end testcodeCrypto007Safe
