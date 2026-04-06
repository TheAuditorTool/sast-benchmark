//! CWE-798: Database connection string with embedded password in const.

// vuln-code-snippet start testcodeHardcodedcreds001
const DB_URL: &str = "postgres://admin:s3cretpass@db:5432/app"; // vuln-code-snippet target-line testcodeHardcodedcreds001

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let table = req.param("table");
    let result = format!("Connected to {} for table {}", DB_URL, table);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds001
