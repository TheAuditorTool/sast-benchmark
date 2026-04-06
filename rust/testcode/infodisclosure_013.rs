//! CWE-209: Database table name included in error response sent to client.

// vuln-code-snippet start testcodeInfodisclosure013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let db_err = format!("Query failed on table 'users': no row found for id={}", id);
    super::shared::BenchmarkResponse::error(&db_err) // vuln-code-snippet target-line testcodeInfodisclosure013
}
// vuln-code-snippet end testcodeInfodisclosure013
