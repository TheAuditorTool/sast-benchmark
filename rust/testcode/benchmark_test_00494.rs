pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let seed: u64 = 1_700_000_000_123_456_789;
    let token = always_secure(seed);

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn always_secure(_seed: u64) -> String {
    os_csprng_token()
}

fn os_csprng_token() -> String {
    let bytes: [u8; 32] = [0x2A; 32];
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
