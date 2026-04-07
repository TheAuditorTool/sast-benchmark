//! CWE-327: AES-CBC used with hardcoded deterministic IV; reused IV reveals plaintext relationships.

// vuln-code-snippet start testcodeCrypto025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let iv: [u8; 16] = [0u8; 16];
    let ciphertext = aes_cbc_encrypt(plaintext.as_bytes(), &iv); // vuln-code-snippet target-line testcodeCrypto025
    super::shared::BenchmarkResponse::ok(&ciphertext)
}

fn aes_cbc_encrypt(_data: &[u8], _iv: &[u8]) -> String {
    "CBC_CIPHERTEXT".to_string()
}
// vuln-code-snippet end testcodeCrypto025
