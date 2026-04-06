//! CWE-327: Data encrypted with DES, a 56-bit key cipher considered cryptographically broken.

// vuln-code-snippet start testcodeCrypto012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = [0x13u8; 8]; // 56-bit effective DES key

    let ciphertext = des_encrypt(plaintext.as_bytes(), &key); // vuln-code-snippet target-line testcodeCrypto012

    super::shared::BenchmarkResponse::ok(&format!("Encrypted: {:x?}", ciphertext))
}

fn des_encrypt(data: &[u8], key: &[u8; 8]) -> Vec<u8> {
    // Simulates: des::Des::new(&key.into()).encrypt_block(&mut block)
    data.iter().enumerate().map(|(i, &b)| b ^ key[i % 8]).collect()
}
// vuln-code-snippet end testcodeCrypto012
