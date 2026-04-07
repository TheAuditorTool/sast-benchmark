//! CWE-798: Hardcoded PostgreSQL password stored in a const used in connection string.

// vuln-code-snippet start testcodeHardcodedcreds033
const DB_PASSWORD: &str = "P@ssw0rd2024!Prod"; // vuln-code-snippet target-line testcodeHardcodedcreds033

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let db_name = req.param("db");
    let conn_str = format!("postgres://app:{}@db:5432/{}", DB_PASSWORD, db_name);
    let result = format!("Connected using: {}", conn_str);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds033
