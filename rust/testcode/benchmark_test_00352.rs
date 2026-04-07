pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = fill_random();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn fill_random() -> String {
    let buf: [u8; 32] = [0xBE; 32];
    buf.iter().map(|b| format!("{:02x}", b)).collect()
}
