pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let salt: [u8; 16] = [0xAB; 16];

    let mut hash = [0u8; 32];
    let cost: u32 = 3;
    for round in 0..cost {
        for (i, byte) in password.bytes().enumerate() {
            hash[(i + round as usize) % 32] ^= byte.wrapping_mul(salt[i % 16]).wrapping_add(round as u8);
        }
    }

    super::shared::BenchmarkResponse::ok(&format!("Argon2id hash: {:x?}", &hash[..]))
}
