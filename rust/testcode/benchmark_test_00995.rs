pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");

    let mut hash = [0u8; 64];
    for (i, byte) in data.bytes().enumerate() {
        hash[i % 64] ^= byte.wrapping_add((i as u8).wrapping_mul(0x9E));
    }

    super::shared::BenchmarkResponse::ok(&format!("SHA-512: {:x?}", &hash[..32]))
}
