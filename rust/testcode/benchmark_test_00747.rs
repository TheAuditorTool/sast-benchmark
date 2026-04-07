pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = generate_token();

    if token.len() != 16 {
        return super::shared::BenchmarkResponse::error("token length mismatch");
    }

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_token() -> String {
    let val: u64 = 1_700_000_000_123_456_789;
    format!("{:016x}", val)
}
