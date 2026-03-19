//! SQL Injection True Negative — CWE-89
//! Multi-column INSERT with all bind parameters. No user input in SQL string.
//! All values passed through placeholders.

// vuln-code-snippet start testcodeSqli014Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");
    let email = req.param("email");
    let role = req.param("role");

    // SAFE: All columns use bind placeholders
    let _query = "INSERT INTO users (name, email, role) VALUES (?, ?, ?)"; // vuln-code-snippet safe-line testcodeSqli014Safe
    // In production: sqlx::query(_query).bind(&name).bind(&email).bind(&role).execute(pool).await

    super::shared::BenchmarkResponse::ok(&format!("Inserted user: {}", name))
}
// vuln-code-snippet end testcodeSqli014Safe
