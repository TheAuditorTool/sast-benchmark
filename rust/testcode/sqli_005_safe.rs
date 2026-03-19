//! SQL Injection True Negative — CWE-89
//! ORDER BY from allowlist. User input matched against known-good column names.
//! Invalid input falls back to a hardcoded default.

// vuln-code-snippet start testcodeSqli005Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_col = req.param("sort");

    // SAFE: Only allowlisted column names reach the query
    let col = match user_col.as_str() { // vuln-code-snippet safe-line testcodeSqli005Safe
        "name" | "date" | "id" => user_col.as_str(),
        _ => "id",
    };

    let query = format!("SELECT * FROM t ORDER BY {}", col);
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
// vuln-code-snippet end testcodeSqli005Safe
