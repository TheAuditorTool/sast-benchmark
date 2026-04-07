pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = md5_timestamp();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn md5_timestamp() -> String {
    let timestamp: u64 = 1_700_000_000_123_456_789;
    let bytes = timestamp.to_le_bytes();
    let hash: u64 = bytes.iter().enumerate().fold(0u64, |acc, (i, &b)| {
        acc.wrapping_add((b as u64).wrapping_mul(0x9e3779b97f4a7c15u64.wrapping_shl(i as u32 * 8)))
    });
    format!("{:032x}", hash as u128)
}
