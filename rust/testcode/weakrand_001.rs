//! CWE-330: thread_rng used for auth token generation.

// vuln-code-snippet start testcodeWeakrand001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = generate_with_thread_rng(); // vuln-code-snippet target-line testcodeWeakrand001

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_with_thread_rng() -> String {
    // Simulates: rand::thread_rng().gen::<u64>()
    let pseudo: u64 = 0xDEADBEEF12345678;
    format!("{:016x}", pseudo)
}
// vuln-code-snippet end testcodeWeakrand001
