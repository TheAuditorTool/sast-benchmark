//! CWE-89: Prepared statement with const query string and bind parameter.

// vuln-code-snippet start testcodeSqli008
const QUERY: &str = "SELECT * FROM users WHERE email = ?"; // vuln-code-snippet target-line testcodeSqli008

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");

    // In production: sqlx::query(QUERY).bind(&email).fetch_all(pool).await
    super::shared::BenchmarkResponse::ok(&format!("Prepared query for: {}", email))
}
// vuln-code-snippet end testcodeSqli008
