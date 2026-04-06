//! CWE-327: Block cipher used in ECB mode; identical plaintext blocks produce identical ciphertext.

// vuln-code-snippet start testcodeCrypto014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = [0x2Bu8; 16]; // 128-bit AES key

    let ciphertext = aes_ecb_encrypt(plaintext.as_bytes(), &key); // vuln-code-snippet target-line testcodeCrypto014

    super::shared::BenchmarkResponse::ok(&format!("ECB ciphertext: {:x?}", ciphertext))
}

fn aes_ecb_encrypt(data: &[u8], key: &[u8; 16]) -> Vec<u8> {
    // Simulates: aes::Aes128::new(&key.into()).encrypt_block(&mut block) in ECB mode (no IV)
    let mut out = Vec::new();
    for chunk in data.chunks(16) {
        let mut block = [0u8; 16];
        for (i, &b) in chunk.iter().enumerate() {
            block[i] = b ^ key[i];
        }
        out.extend_from_slice(&block);
    }
    out
}
// vuln-code-snippet end testcodeCrypto014
