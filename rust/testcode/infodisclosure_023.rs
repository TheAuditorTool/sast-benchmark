//! CWE-209: SQL query error including table and column names exposed in response body.

// vuln-code-snippet start testcodeInfodisclosure023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("id");
    match execute_sql(&user_id) {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => super::shared::BenchmarkResponse::error(&e), // vuln-code-snippet target-line testcodeInfodisclosure023
    }
}

fn execute_sql(_id: &str) -> Result<String, String> {
    Err("ERROR: column \"secret_salary\" of table \"employees\" does not exist".to_string())
}
// vuln-code-snippet end testcodeInfodisclosure023
