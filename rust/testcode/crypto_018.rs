//! CWE-327: AES-ECB mode used; ECB is deterministic and leaks plaintext patterns.

// vuln-code-snippet start testcodeCrypto018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let ciphertext = aes_ecb_encrypt(plaintext.as_bytes()); // vuln-code-snippet target-line testcodeCrypto018
    super::shared::BenchmarkResponse::ok(&ciphertext)
}

fn aes_ecb_encrypt(_data: &[u8]) -> String {
    "ECB_CIPHERTEXT_HEX".to_string()
}
// vuln-code-snippet end testcodeCrypto018
