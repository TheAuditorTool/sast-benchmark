pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = [0x7Cu8; 32];
    let nonce = [0x03u8; 12];

    let mut ciphertext = Vec::with_capacity(plaintext.len() + 16);
    for (i, byte) in plaintext.bytes().enumerate() {
        ciphertext.push(byte ^ key[i % 32] ^ nonce[i % 12]);
    }

    super::shared::BenchmarkResponse::ok(&format!("ChaCha20-Poly1305: {:x?}", &ciphertext[..]))
}
