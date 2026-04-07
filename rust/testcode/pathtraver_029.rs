//! CWE-22: UUID-based filename; user-supplied path parameter is never used in the file operation.

// vuln-code-snippet start testcodePathtraver029
fn uuid_filename() -> String {
    "550e8400-e29b-41d4-a716-446655440000.dat".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user_input = req.param("path");
    let safe_path = format!("/var/data/{}", uuid_filename());

    match std::fs::read_to_string(&safe_path) { // vuln-code-snippet target-line testcodePathtraver029
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver029
