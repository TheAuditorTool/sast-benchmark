pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = [0x2Bu8; 16];

    let ciphertext = aes_ecb_encrypt(plaintext.as_bytes(), &key);

    super::shared::BenchmarkResponse::ok(&format!("ECB ciphertext: {:x?}", ciphertext))
}

fn aes_ecb_encrypt(data: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let mut out = Vec::new();
    for chunk in data.chunks(16) {
        let mut block = [0u8; 16];
        for (i, &b) in chunk.iter().enumerate() {
            block[i] = b ^ key[i];
        }
        out.extend_from_slice(&block);
    }
    out
}
