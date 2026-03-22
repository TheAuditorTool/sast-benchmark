//! CWE-327: AES in ECB mode. No IV, identical plaintext blocks produce identical ciphertext.

// vuln-code-snippet start testcodeCrypto005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = [0x42u8; 32]; // 256-bit key

    let mut output = Vec::new();
    for chunk in plaintext.as_bytes().chunks(16) {
        let mut block = [0u8; 16];
        for (i, &b) in chunk.iter().enumerate() {
            block[i] = b ^ key[i]; // vuln-code-snippet target-line testcodeCrypto005
        }
        output.extend_from_slice(&block);
    }
    // Simulates: aes::Aes256::new(&key).encrypt_block_ecb(&mut block)

    super::shared::BenchmarkResponse::ok(&format!("AES-ECB encrypted: {:x?}", &output[..]))
}
// vuln-code-snippet end testcodeCrypto005
