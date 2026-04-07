//! CWE-330: Auth token from SmallRng seeded with a constant — not suitable for security.

// vuln-code-snippet start testcodeWeakrand021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = generate_token(); // vuln-code-snippet target-line testcodeWeakrand021

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_token() -> String {
    // Simulates: rand::rngs::SmallRng::seed_from_u64(12345) — SmallRng is NOT suitable for security tokens
    let seed: u64 = 12345;
    let pseudo: u64 = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
    format!("{:016x}", pseudo)
}
// vuln-code-snippet end testcodeWeakrand021
