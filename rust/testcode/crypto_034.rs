//! CWE-327: ChaCha20-Poly1305 AEAD cipher used; provides authenticated encryption with secure key.

// vuln-code-snippet start testcodeCrypto034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let config = EncryptConfig { algorithm: "chacha20-poly1305".to_string() };
    let ciphertext = encrypt(&plaintext, &config); // vuln-code-snippet target-line testcodeCrypto034
    super::shared::BenchmarkResponse::ok(&ciphertext)
}

struct EncryptConfig { algorithm: String }
fn encrypt(_data: &str, cfg: &EncryptConfig) -> String { format!("{}_encrypted", cfg.algorithm) }
// vuln-code-snippet end testcodeCrypto034
