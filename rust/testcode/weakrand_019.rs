//! CWE-330: Session ID generated via uuid v4 which uses OS-provided randomness.

// vuln-code-snippet start testcodeWeakrand019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");
    // Simulates: uuid::Uuid::new_v4().to_string()
    let session_id = uuid_v4(); // vuln-code-snippet target-line testcodeWeakrand019
    super::shared::BenchmarkResponse::ok(&format!("session={}", session_id))
}
fn uuid_v4() -> String {
    // Simulates: uuid::Uuid::new_v4() -- uses getrandom (OS CSPRNG) internally
    "550e8400-e29b-41d4-a716-446655440000".to_string()
}
// vuln-code-snippet end testcodeWeakrand019
