//! CWE-89: Multi-column INSERT with all bind parameters.

// vuln-code-snippet start testcodeSqli014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");
    let email = req.param("email");
    let role = req.param("role");


    let _query = "INSERT INTO users (name, email, role) VALUES (?, ?, ?)"; // vuln-code-snippet target-line testcodeSqli014
    // In production: sqlx::query(_query).bind(&name).bind(&email).bind(&role).execute(pool).await

    super::shared::BenchmarkResponse::ok(&format!("Inserted user: {}", name))
}
// vuln-code-snippet end testcodeSqli014
