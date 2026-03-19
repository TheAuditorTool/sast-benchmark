//! SQL Injection True Positive — CWE-89
//! LIKE wildcard injection. User search term interpolated into LIKE clause.
//! Attacker can inject SQL via the search parameter.

// vuln-code-snippet start testcodeSqli004Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_search = req.param("search");

    // VULNERABLE: User input directly in LIKE clause via format!()
    let query = format!("SELECT * FROM t WHERE name LIKE '%{}%'", user_search); // vuln-code-snippet vuln-line testcodeSqli004Vulnerable

    // In production: sqlx::query(&query).fetch_all(pool).await
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
// vuln-code-snippet end testcodeSqli004Vulnerable
