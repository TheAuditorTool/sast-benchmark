//! SQL Injection True Negative — CWE-89
//! Prepared statement with a const query string. User input bound via parameter.
//! The SQL string itself is a compile-time constant.

// vuln-code-snippet start testcodeSqli008Safe
const QUERY: &str = "SELECT * FROM users WHERE email = ?"; // vuln-code-snippet safe-line testcodeSqli008Safe

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");

    // In production: sqlx::query(QUERY).bind(&email).fetch_all(pool).await
    super::shared::BenchmarkResponse::ok(&format!("Prepared query for: {}", email))
}
// vuln-code-snippet end testcodeSqli008Safe
