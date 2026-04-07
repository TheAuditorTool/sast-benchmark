//! CWE-330: Timestamp token unconditionally overwritten by OsRng token before use — secure.

// vuln-code-snippet start testcodeWeakrand038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let mut token = timestamp_token(); // vuln-code-snippet target-line testcodeWeakrand038
    token = os_random_token();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn timestamp_token() -> String {
    // Simulates: SystemTime::now().duration_since(EPOCH).as_nanos() as u64
    let val: u64 = 1_700_000_000_123_456_789;
    format!("{:016x}", val)
}

fn os_random_token() -> String {
    // Simulates: rand::rngs::OsRng.gen::<[u8; 32]>() — OS CSPRNG
    let bytes: [u8; 32] = [0xD4; 32];
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand038
