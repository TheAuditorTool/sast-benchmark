//! CWE-330: Thread ID used as randomness source — deterministic in single-thread scenario.

// vuln-code-snippet start testcodeWeakrand028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = generate_token(); // vuln-code-snippet target-line testcodeWeakrand028

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_token() -> String {
    // Simulates: std::thread::current().id() as u64 — deterministic in single-thread scenario
    let thread_id: u64 = 1;
    let val = thread_id.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
    format!("{:016x}", val)
}
// vuln-code-snippet end testcodeWeakrand028
