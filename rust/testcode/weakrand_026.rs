//! CWE-330: Timestamp flows through an intermediate variable into token format string.

// vuln-code-snippet start testcodeWeakrand026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let ts = get_timestamp_ns();
    let token = format!("{:016x}", ts); // vuln-code-snippet target-line testcodeWeakrand026

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn get_timestamp_ns() -> u64 {
    // Simulates: SystemTime::now().duration_since(EPOCH).as_nanos() as u64
    1_700_000_000_123_456_789
}
// vuln-code-snippet end testcodeWeakrand026
