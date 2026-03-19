//! Weak Cryptography True Positive — CWE-327
//! AES in ECB mode — no IV, identical plaintext blocks produce identical
//! ciphertext blocks. Leaks data patterns (see: ECB penguin).

// vuln-code-snippet start testcodeCrypto005Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = [0x42u8; 32]; // 256-bit key

    // VULNERABLE: ECB mode preserves plaintext patterns — no semantic security
    let mut output = Vec::new();
    for chunk in plaintext.as_bytes().chunks(16) {
        let mut block = [0u8; 16];
        for (i, &b) in chunk.iter().enumerate() {
            block[i] = b ^ key[i]; // vuln-code-snippet vuln-line testcodeCrypto005Vulnerable
        }
        output.extend_from_slice(&block);
    }
    // Simulates: aes::Aes256::new(&key).encrypt_block_ecb(&mut block)

    super::shared::BenchmarkResponse::ok(&format!("AES-ECB encrypted: {:x?}", &output[..]))
}
// vuln-code-snippet end testcodeCrypto005Vulnerable
