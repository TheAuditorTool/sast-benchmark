//! CWE-330: Constant prefix prepended to timestamp token — still predictable despite decoration.

// vuln-code-snippet start testcodeWeakrand027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let timestamp = get_timestamp_ns();
    let token = format!("tok_{:016x}", timestamp); // vuln-code-snippet target-line testcodeWeakrand027

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn get_timestamp_ns() -> u64 {
    // Simulates: SystemTime::now().duration_since(EPOCH).as_nanos() as u64
    1_700_000_000_123_456_789
}
// vuln-code-snippet end testcodeWeakrand027
