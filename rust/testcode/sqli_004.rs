//! CWE-89: LIKE clause with user search term interpolated via format!().

// vuln-code-snippet start testcodeSqli004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_search = req.param("search");

    let query = format!("SELECT * FROM t WHERE name LIKE '%{}%'", user_search); // vuln-code-snippet target-line testcodeSqli004

    // In production: sqlx::query(&query).fetch_all(pool).await
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
// vuln-code-snippet end testcodeSqli004
