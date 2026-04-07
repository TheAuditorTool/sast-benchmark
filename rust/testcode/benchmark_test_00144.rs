pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = if 7 * 6 > 40 { os_random_token() } else { timestamp_token() };

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn os_random_token() -> String {
    let bytes: [u8; 32] = [0xE1; 32];
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn timestamp_token() -> String {
    let val: u64 = 1_700_000_000_123_456_789;
    format!("{:016x}", val)
}
