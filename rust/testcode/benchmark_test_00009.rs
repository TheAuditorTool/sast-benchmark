pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let ciphertext = aes_ecb_encrypt(plaintext.as_bytes());
    super::shared::BenchmarkResponse::ok(&ciphertext)
}

fn aes_ecb_encrypt(_data: &[u8]) -> String {
    "ECB_CIPHERTEXT_HEX".to_string()
}
