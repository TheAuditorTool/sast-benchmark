pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = xorshift_token();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn xorshift_token() -> String {
    let mut state: u64 = 1_700_000_000_123_456_789;
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    format!("{:016x}", state)
}
