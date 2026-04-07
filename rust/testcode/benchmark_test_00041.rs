pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key_bytes: [u8; 8] = [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0x01, 0x02];

    let mut cipher_block = [0u8; 8];
    for (i, byte) in plaintext.bytes().take(8).enumerate() {
        cipher_block[i] = byte ^ key_bytes[i];
    }

    super::shared::BenchmarkResponse::ok(&format!("DES encrypted: {:x?}", &cipher_block[..]))
}
