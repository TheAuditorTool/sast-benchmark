//! CWE-330: Process ID used as random seed — predictable and low-entropy.

// vuln-code-snippet start testcodeWeakrand025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = generate_token(); // vuln-code-snippet target-line testcodeWeakrand025

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_token() -> String {
    // Simulates: std::process::id() as u64 used as seed — predictable
    let pid: u64 = 1234;
    let val = pid.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
    format!("{:016x}", val)
}
// vuln-code-snippet end testcodeWeakrand025
