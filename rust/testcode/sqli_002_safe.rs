//! SQL Injection True Negative — CWE-89
//! Parameterized query with bind placeholder. User input never enters the query string.
//! Isomorphic to sqli_001_vulnerable — same data flow, safe API.

// vuln-code-snippet start testcodeSqli002Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");

    // SAFE: Parameterized query — user input bound separately, never in SQL string
    let _query = "SELECT * FROM users WHERE id = ?"; // vuln-code-snippet safe-line testcodeSqli002Safe
    // In production: sqlx::query(query).bind(&id).fetch_all(pool).await

    super::shared::BenchmarkResponse::ok(&format!("Parameterized query for id: {}", id))
}
// vuln-code-snippet end testcodeSqli002Safe
