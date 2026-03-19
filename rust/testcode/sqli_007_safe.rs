//! SQL Injection True Negative — CWE-89
//! Integer-only ID. User input parsed to i64 before use in query.
//! Non-numeric input causes a hard error, never reaches the query.

// vuln-code-snippet start testcodeSqli007Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.param("id");

    // SAFE: Parse to integer — non-numeric input rejected
    let id: i64 = match raw.parse::<i64>() { // vuln-code-snippet safe-line testcodeSqli007Safe
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid id"),
    };

    let query = format!("SELECT * FROM users WHERE id = {}", id);
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
// vuln-code-snippet end testcodeSqli007Safe
