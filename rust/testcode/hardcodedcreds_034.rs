//! CWE-798: Database password read from environment variable at runtime.

// vuln-code-snippet start testcodeHardcodedcreds034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let db_name = req.param("db");
    let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"); // vuln-code-snippet target-line testcodeHardcodedcreds034
    let conn_str = format!("postgres://app:{}@db:5432/{}", db_password, db_name);
    super::shared::BenchmarkResponse::ok(&conn_str)
}
// vuln-code-snippet end testcodeHardcodedcreds034
