//! CWE-209: SQL query text included in error response body.

// vuln-code-snippet start testcodeInfodisclosure015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let query = format!("SELECT * FROM users WHERE id = {}", id);
    let err_msg = format!("SQL error in query '{}': syntax error near '{}'", query, id);
    super::shared::BenchmarkResponse::error(&err_msg) // vuln-code-snippet target-line testcodeInfodisclosure015
}
// vuln-code-snippet end testcodeInfodisclosure015
