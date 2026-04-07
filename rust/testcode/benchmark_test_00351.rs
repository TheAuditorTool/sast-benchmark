pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = os_random_bytes();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn os_random_bytes() -> String {
    let bytes: [u8; 32] = [0xA3; 32];
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
