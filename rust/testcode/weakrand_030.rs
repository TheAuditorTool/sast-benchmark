//! CWE-330: XorShift64 PRNG seeded from current timestamp — not a CSPRNG.

// vuln-code-snippet start testcodeWeakrand030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = xorshift_token(); // vuln-code-snippet target-line testcodeWeakrand030

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn xorshift_token() -> String {
    // Simulates: XorShift64 seeded with current time — not CSPRNG
    let mut state: u64 = 1_700_000_000_123_456_789;
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    format!("{:016x}", state)
}
// vuln-code-snippet end testcodeWeakrand030
