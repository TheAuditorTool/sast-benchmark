pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _client = req.param("client");

    let seed: u64 = 0xDEAD_CAFE_1234_5678;
    let api_key = xorshift_key(seed);

    super::shared::BenchmarkResponse::ok(&format!("api_key={:016x}", api_key))
}

fn xorshift_key(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
}
