//! Weak Random True Negative — CWE-330
//! UUID v4 for session ID. UUID v4 uses 122 bits of cryptographic
//! randomness — infeasible to predict or collide.

// vuln-code-snippet start testcodeWeakrand008Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    // SAFE: UUID v4 uses cryptographic randomness
    let session_id = generate_uuid_v4(); // vuln-code-snippet safe-line testcodeWeakrand008Safe

    super::shared::BenchmarkResponse::ok(&format!("Session: {}", session_id))
}

fn generate_uuid_v4() -> String {
    // Simulates: uuid::Uuid::new_v4().to_string()
    "550e8400-e29b-41d4-a716-446655440000".to_string()
}
// vuln-code-snippet end testcodeWeakrand008Safe
