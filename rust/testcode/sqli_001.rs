//! CWE-89: String concatenation via format!() in SQL query.

// vuln-code-snippet start testcodeSqli001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");

    let query = format!("SELECT * FROM users WHERE id = {}", id); // vuln-code-snippet target-line testcodeSqli001

    // In production: sqlx::query(&query).fetch_all(pool).await
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
// vuln-code-snippet end testcodeSqli001
