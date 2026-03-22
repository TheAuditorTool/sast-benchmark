//! CWE-89: ORDER BY injection. User controls the sort column via format!().

// vuln-code-snippet start testcodeSqli003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_col = req.param("sort");

    let query = format!("SELECT * FROM t ORDER BY {}", user_col); // vuln-code-snippet target-line testcodeSqli003

    // In production: sqlx::query(&query).fetch_all(pool).await
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
// vuln-code-snippet end testcodeSqli003
