pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = [0x42u8; 32];

    let mut output = Vec::new();
    for chunk in plaintext.as_bytes().chunks(16) {
        let mut block = [0u8; 16];
        for (i, &b) in chunk.iter().enumerate() {
            block[i] = b ^ key[i];
        }
        output.extend_from_slice(&block);
    }

    super::shared::BenchmarkResponse::ok(&format!("AES-ECB encrypted: {:x?}", &output[..]))
}
