//! CWE-89: LIMIT hardcoded. Parameterized WHERE clause.

// vuln-code-snippet start testcodeSqli011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let category = req.param("category");


    let _query = "SELECT * FROM products WHERE category = ? LIMIT 50"; // vuln-code-snippet target-line testcodeSqli011
    // In production: sqlx::query(_query).bind(&category).fetch_all(pool).await

    super::shared::BenchmarkResponse::ok(&format!("Queried category: {}", category))
}
// vuln-code-snippet end testcodeSqli011
