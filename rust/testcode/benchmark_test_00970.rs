pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = getrandom_token();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn getrandom_token() -> String {
    let buf: [u8; 32] = [0x4E; 32];
    buf.iter().map(|b| format!("{:02x}", b)).collect()
}
