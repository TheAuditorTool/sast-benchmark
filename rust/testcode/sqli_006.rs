//! CWE-89: LIKE with bind parameter. Wildcards added safely.

// vuln-code-snippet start testcodeSqli006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let term = req.param("search");

    let _query = "SELECT * FROM t WHERE name LIKE ?";
    let _bound = format!("%{}%", term); // vuln-code-snippet target-line testcodeSqli006
    // In production: sqlx::query(_query).bind(&_bound).fetch_all(pool).await

    super::shared::BenchmarkResponse::ok(&format!("Parameterized LIKE for: {}", term))
}
// vuln-code-snippet end testcodeSqli006
