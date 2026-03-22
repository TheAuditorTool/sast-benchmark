//! CWE-89: Stored procedure call with bind parameters.

// vuln-code-snippet start testcodeSqli010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let action = req.param("action");


    let _query = "CALL process_action(?, ?)"; // vuln-code-snippet target-line testcodeSqli010
    // In production: sqlx::query(_query).bind(&user_id).bind(&action).execute(pool).await

    super::shared::BenchmarkResponse::ok(&format!("Called procedure for: {}", user_id))
}
// vuln-code-snippet end testcodeSqli010
