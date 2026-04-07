pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = chacha20_token();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn chacha20_token() -> String {
    let bytes: [u8; 32] = [0x6D; 32];
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
