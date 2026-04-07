pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = generate_chacha20();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_chacha20() -> String {
    let csprng_bytes: [u8; 32] = [0xEF; 32];
    csprng_bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
