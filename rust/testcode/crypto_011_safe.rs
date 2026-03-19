//! Weak Cryptography True Negative — CWE-327
//! AES-256-CBC with random IV. Unlike ECB, CBC chains blocks through
//! XOR with the previous ciphertext block, hiding plaintext patterns.

// vuln-code-snippet start testcodeCrypto011Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = [0x42u8; 32]; // 256-bit key
    let iv = [0xAAu8; 16]; // Random IV in production

    // SAFE: AES-CBC with random IV provides semantic security (unlike ECB)
    let mut prev = iv;
    let mut output = Vec::new();
    for chunk in plaintext.as_bytes().chunks(16) {
        let mut block = [0u8; 16];
        for (i, &b) in chunk.iter().enumerate() {
            block[i] = (b ^ prev[i]) ^ key[i]; // vuln-code-snippet safe-line testcodeCrypto011Safe
        }
        prev = block;
        output.extend_from_slice(&block);
    }
    // Simulates: cbc::Encryptor::<aes::Aes256>::new(&key, &iv).encrypt_padded(&data)

    super::shared::BenchmarkResponse::ok(&format!("AES-CBC encrypted: {:x?}", &output[..]))
}
// vuln-code-snippet end testcodeCrypto011Safe
