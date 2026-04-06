//! CWE-798: Database password loaded from environment variable at runtime.

// vuln-code-snippet start testcodeHardcodedcreds011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let table = req.param("table");
    let db_password = std::env::var("DB_PASSWORD").unwrap_or_default(); // vuln-code-snippet target-line testcodeHardcodedcreds011
    let db_url = format!("postgres://admin:{}@db:5432/app", db_password);
    let result = format!("Connected for table {}", table);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds011
