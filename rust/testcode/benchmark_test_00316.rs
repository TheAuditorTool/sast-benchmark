pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = getrandom_bytes();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn getrandom_bytes() -> String {
    let mut buf = [0u8; 32];
    buf.iter_mut().enumerate().for_each(|(i, b)| *b = (i as u8).wrapping_add(0xA0));
    buf.iter().map(|b| format!("{:02x}", b)).collect()
}
