//! CWE-330: Dead-code branch — constant condition always selects OsRng path, never timestamp.

// vuln-code-snippet start testcodeWeakrand036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = if 7 * 6 > 40 { os_random_token() } else { timestamp_token() }; // vuln-code-snippet target-line testcodeWeakrand036

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn os_random_token() -> String {
    // Simulates: rand::rngs::OsRng.gen::<[u8; 32]>() — OS CSPRNG
    let bytes: [u8; 32] = [0xE1; 32];
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn timestamp_token() -> String {
    // Simulates: SystemTime::now().duration_since(EPOCH).as_nanos() as u64
    let val: u64 = 1_700_000_000_123_456_789;
    format!("{:016x}", val)
}
// vuln-code-snippet end testcodeWeakrand036
