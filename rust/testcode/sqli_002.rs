//! CWE-89: Parameterized query with bind placeholder.

// vuln-code-snippet start testcodeSqli002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");

    let _query = "SELECT * FROM users WHERE id = ?"; // vuln-code-snippet target-line testcodeSqli002
    // In production: sqlx::query(query).bind(&id).fetch_all(pool).await

    super::shared::BenchmarkResponse::ok(&format!("Parameterized query for id: {}", id))
}
// vuln-code-snippet end testcodeSqli002
