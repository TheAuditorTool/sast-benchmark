pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let timestamp = get_timestamp_ns();
    let token = format!("tok_{:016x}", timestamp);

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn get_timestamp_ns() -> u64 {
    1_700_000_000_123_456_789
}
