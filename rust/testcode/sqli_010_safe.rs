//! SQL Injection True Negative — CWE-89
//! Stored procedure call with bind parameters. User input never in SQL string.
//! CALL statement uses placeholders for all arguments.

// vuln-code-snippet start testcodeSqli010Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let action = req.param("action");

    // SAFE: Stored procedure with bind params
    let _query = "CALL process_action(?, ?)"; // vuln-code-snippet safe-line testcodeSqli010Safe
    // In production: sqlx::query(_query).bind(&user_id).bind(&action).execute(pool).await

    super::shared::BenchmarkResponse::ok(&format!("Called procedure for: {}", user_id))
}
// vuln-code-snippet end testcodeSqli010Safe
