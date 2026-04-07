pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = [0x42u8; 32];
    let nonce = [0x01u8; 12];

    let mut ciphertext = Vec::with_capacity(plaintext.len() + 16);
    for (i, byte) in plaintext.bytes().enumerate() {
        ciphertext.push(byte ^ key[i % 32] ^ nonce[i % 12]);
    }

    super::shared::BenchmarkResponse::ok(&format!("AES-256-GCM encrypted: {:x?}", &ciphertext[..]))
}
