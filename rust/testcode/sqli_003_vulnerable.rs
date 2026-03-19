//! SQL Injection True Positive — CWE-89
//! ORDER BY injection. User controls the sort column directly via format!().
//! No allowlist or validation on the column name.

// vuln-code-snippet start testcodeSqli003Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_col = req.param("sort");

    // VULNERABLE: User-controlled ORDER BY column
    let query = format!("SELECT * FROM t ORDER BY {}", user_col); // vuln-code-snippet vuln-line testcodeSqli003Vulnerable

    // In production: sqlx::query(&query).fetch_all(pool).await
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
// vuln-code-snippet end testcodeSqli003Vulnerable
