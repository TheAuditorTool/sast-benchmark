//! CWE-330: Auth token derived from current timestamp nanoseconds — predictable.

// vuln-code-snippet start testcodeWeakrand020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = generate_token(); // vuln-code-snippet target-line testcodeWeakrand020

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_token() -> String {
    // Simulates: SystemTime::now().duration_since(EPOCH).as_nanos() as u64
    let val: u64 = 1_700_000_000_123_456_789;
    format!("{:016x}", val)
}
// vuln-code-snippet end testcodeWeakrand020
