//! CWE-327: Encryption key is only 64 bits; keys shorter than 128 bits are insecure.

// vuln-code-snippet start testcodeCrypto019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let short_key: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let ciphertext = encrypt_with_key(data.as_bytes(), &short_key); // vuln-code-snippet target-line testcodeCrypto019
    super::shared::BenchmarkResponse::ok(&ciphertext)
}

fn encrypt_with_key(_data: &[u8], _key: &[u8]) -> String {
    "CIPHERTEXT".to_string()
}
// vuln-code-snippet end testcodeCrypto019
