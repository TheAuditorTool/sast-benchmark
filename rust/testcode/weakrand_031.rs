//! CWE-330: Length check on generated token passes, but underlying source is still timestamp-based.

// vuln-code-snippet start testcodeWeakrand031
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = generate_token(); // vuln-code-snippet target-line testcodeWeakrand031

    if token.len() != 16 {
        return super::shared::BenchmarkResponse::error("token length mismatch");
    }

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_token() -> String {
    // Simulates: SystemTime::now().duration_since(EPOCH).as_nanos() as u64
    let val: u64 = 1_700_000_000_123_456_789;
    format!("{:016x}", val)
}
// vuln-code-snippet end testcodeWeakrand031
