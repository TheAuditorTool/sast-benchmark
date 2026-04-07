pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = 0x42u8;
    let encrypted: Vec<u8> = plaintext.bytes().map(|b| b ^ key).collect();
    super::shared::BenchmarkResponse::ok(&format!("encrypted_len={}", encrypted.len()))
}
