pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = [0xA3u8; 32];

    let sealed = aead_seal(plaintext.as_bytes(), &key);

    super::shared::BenchmarkResponse::ok(&format!("Sealed: {:x?}", sealed))
}

fn aead_seal(data: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let mut nonce = [0u8; 12];
    for (i, b) in nonce.iter_mut().enumerate() {
        *b = (i as u8) ^ key[i % 32];
    }
    let mut out = nonce.to_vec();
    out.extend(data.iter().zip(key.iter().cycle()).map(|(&b, &k)| b ^ k));
    out
}
