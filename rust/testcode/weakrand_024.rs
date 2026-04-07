//! CWE-330: MD5 of a timestamp used as session token — broken hash of predictable input.

// vuln-code-snippet start testcodeWeakrand024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = md5_timestamp(); // vuln-code-snippet target-line testcodeWeakrand024

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn md5_timestamp() -> String {
    // Simulates: md5::compute(timestamp.to_le_bytes()) — MD5 of predictable timestamp
    let timestamp: u64 = 1_700_000_000_123_456_789;
    let bytes = timestamp.to_le_bytes();
    let hash: u64 = bytes.iter().enumerate().fold(0u64, |acc, (i, &b)| {
        acc.wrapping_add((b as u64).wrapping_mul(0x9e3779b97f4a7c15u64.wrapping_shl(i as u32 * 8)))
    });
    format!("{:032x}", hash as u128)
}
// vuln-code-snippet end testcodeWeakrand024
