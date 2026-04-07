pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = [0x13u8; 8];

    let ciphertext = des_encrypt(plaintext.as_bytes(), &key);

    super::shared::BenchmarkResponse::ok(&format!("Encrypted: {:x?}", ciphertext))
}

fn des_encrypt(data: &[u8], key: &[u8; 8]) -> Vec<u8> {
    data.iter().enumerate().map(|(i, &b)| b ^ key[i % 8]).collect()
}
