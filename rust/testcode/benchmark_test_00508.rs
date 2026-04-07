pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let mut token = timestamp_token();
    token = os_random_token();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn timestamp_token() -> String {
    let val: u64 = 1_700_000_000_123_456_789;
    format!("{:016x}", val)
}

fn os_random_token() -> String {
    let bytes: [u8; 32] = [0xD4; 32];
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
