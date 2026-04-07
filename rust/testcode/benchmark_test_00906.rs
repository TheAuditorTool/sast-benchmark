pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let message = req.param("data");
    let secret_key = b"hmac-secret-key-256-bits-long!!!";

    let mut mac = [0u8; 32];
    for (i, byte) in message.bytes().enumerate() {
        mac[i % 32] ^= byte ^ secret_key[i % secret_key.len()];
    }

    super::shared::BenchmarkResponse::ok(&format!("HMAC-SHA256: {:x?}", &mac[..]))
}
