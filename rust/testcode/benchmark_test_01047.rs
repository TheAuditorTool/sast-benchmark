pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = generate_token();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_token() -> String {
    let val: u64 = 1_700_000_000_123_456_789;
    format!("{:016x}", val)
}
