//! CWE-209: Custom error type returning generic message without internal details.

// vuln-code-snippet start testcodeInfodisclosure018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    match fetch_record(&id) {
        Ok(data) => super::shared::BenchmarkResponse::ok(&data),
        Err(_) => super::shared::BenchmarkResponse::error("An error occurred"), // vuln-code-snippet target-line testcodeInfodisclosure018
    }
}
fn fetch_record(id: &str) -> Result<String, String> {
    if id.is_empty() { Err("DB connection failed at 10.0.1.5:5432".into()) } else { Ok(format!("record_{}", id)) }
}
// vuln-code-snippet end testcodeInfodisclosure018
