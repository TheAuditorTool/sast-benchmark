//! CWE-330: UUID v4 token using OS CSPRNG internally — secure.

// vuln-code-snippet start testcodeWeakrand034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = uuid_v4(); // vuln-code-snippet target-line testcodeWeakrand034

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn uuid_v4() -> String {
    // Simulates: uuid::Uuid::new_v4() — uses getrandom (OS CSPRNG) internally
    "550e8400-e29b-41d4-a716-446655440000".to_string()
}
// vuln-code-snippet end testcodeWeakrand034
