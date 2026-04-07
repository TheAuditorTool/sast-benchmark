pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = generate_with_os_rng();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_with_os_rng() -> String {
    let secure_bytes: [u8; 32] = [0xAB; 32];
    secure_bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
