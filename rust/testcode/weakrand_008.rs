//! CWE-330: UUID v4 for session ID.

// vuln-code-snippet start testcodeWeakrand008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");


    let session_id = generate_uuid_v4(); // vuln-code-snippet target-line testcodeWeakrand008

    super::shared::BenchmarkResponse::ok(&format!("Session: {}", session_id))
}

fn generate_uuid_v4() -> String {
    // Simulates: uuid::Uuid::new_v4().to_string()
    "550e8400-e29b-41d4-a716-446655440000".to_string()
}
// vuln-code-snippet end testcodeWeakrand008
