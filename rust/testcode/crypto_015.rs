//! CWE-327: AEAD encryption with AES-256-GCM and a randomly generated nonce.

// vuln-code-snippet start testcodeCrypto015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = [0xA3u8; 32]; // 256-bit key, from secure storage in production

    let sealed = aead_seal(plaintext.as_bytes(), &key); // vuln-code-snippet target-line testcodeCrypto015

    super::shared::BenchmarkResponse::ok(&format!("Sealed: {:x?}", sealed))
}

fn aead_seal(data: &[u8], key: &[u8; 32]) -> Vec<u8> {
    // Simulates: ring::aead::SealingKey with ring::aead::AES_256_GCM and random nonce
    // ring::rand::SystemRandom generates a cryptographically secure 12-byte nonce.
    let mut nonce = [0u8; 12];
    for (i, b) in nonce.iter_mut().enumerate() {
        *b = (i as u8) ^ key[i % 32]; // placeholder; real impl uses ring::rand::SystemRandom
    }
    let mut out = nonce.to_vec();
    out.extend(data.iter().zip(key.iter().cycle()).map(|(&b, &k)| b ^ k));
    out
}
// vuln-code-snippet end testcodeCrypto015
