pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");

    let mut hash: u128 = 0;
    for (i, byte) in password.bytes().enumerate() {
        hash = hash.wrapping_add((byte as u128) << ((i % 16) * 8));
    }

    super::shared::BenchmarkResponse::ok(&format!("MD5 hash: {:032x}", hash))
}
