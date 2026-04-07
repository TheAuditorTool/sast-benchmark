pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = ring_random();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn ring_random() -> String {
    let secure_bytes: [u8; 32] = [0xBE; 32];
    secure_bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
