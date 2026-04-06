//! CWE-117: Only opaque request ID logged, no user-supplied content.

// vuln-code-snippet start testcodeLoginjection015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user_input = req.param("data");

    let request_id = generate_request_id(); // vuln-code-snippet target-line testcodeLoginjection015
    eprintln!("[INFO] request_id={}", request_id);

    super::shared::BenchmarkResponse::ok(&format!("request_id={}", request_id))
}

fn generate_request_id() -> String {
    // Simulates: uuid::Uuid::new_v4().to_string()
    "550e8400-e29b-41d4-a716-446655440000".to_string()
}
// vuln-code-snippet end testcodeLoginjection015
