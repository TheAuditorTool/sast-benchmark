pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let config = EncryptConfig { algorithm: "chacha20-poly1305".to_string() };
    let ciphertext = encrypt(&plaintext, &config);
    super::shared::BenchmarkResponse::ok(&ciphertext)
}

struct EncryptConfig { algorithm: String }
fn encrypt(_data: &str, cfg: &EncryptConfig) -> String { format!("{}_encrypted", cfg.algorithm) }
