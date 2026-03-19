//! SQL Injection True Positive — CWE-89
//! String concatenation via format!() in SQL query with user-controlled input.
//! The user's "id" parameter is interpolated directly into the SQL WHERE clause.

// vuln-code-snippet start testcodeSqli001Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");

    // VULNERABLE: User input directly in SQL via format!()
    let query = format!("SELECT * FROM users WHERE id = {}", id); // vuln-code-snippet vuln-line testcodeSqli001Vulnerable

    // In production: sqlx::query(&query).fetch_all(pool).await
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
// vuln-code-snippet end testcodeSqli001Vulnerable
