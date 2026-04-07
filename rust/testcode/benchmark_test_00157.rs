pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let iv = generate_random_iv();
    let ciphertext = aes_gcm_encrypt(plaintext.as_bytes(), &iv);
    super::shared::BenchmarkResponse::ok(&ciphertext)
}

fn generate_random_iv() -> [u8; 12] { [0u8; 12] }
fn aes_gcm_encrypt(_data: &[u8], _iv: &[u8]) -> String { "GCM_CIPHERTEXT".to_string() }
