//! SQL Injection True Negative — CWE-89
//! LIMIT hardcoded. User cannot control the LIMIT value.
//! Only the parameterized WHERE clause uses user input.

// vuln-code-snippet start testcodeSqli011Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let category = req.param("category");

    // SAFE: LIMIT is hardcoded, user input is bound via parameter
    let _query = "SELECT * FROM products WHERE category = ? LIMIT 50"; // vuln-code-snippet safe-line testcodeSqli011Safe
    // In production: sqlx::query(_query).bind(&category).fetch_all(pool).await

    super::shared::BenchmarkResponse::ok(&format!("Queried category: {}", category))
}
// vuln-code-snippet end testcodeSqli011Safe
