//! SQL Injection True Negative — CWE-89
//! Input regex validated. Only alphanumeric characters allowed through.
//! Anything with special chars is rejected before reaching the query.

// vuln-code-snippet start testcodeSqli012Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");

    // SAFE: Regex rejects any non-alphanumeric input
    if !username.chars().all(|c| c.is_alphanumeric()) { // vuln-code-snippet safe-line testcodeSqli012Safe
        return super::shared::BenchmarkResponse::bad_request("Invalid username");
    }

    let query = format!("SELECT * FROM users WHERE username = '{}'", username);
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
// vuln-code-snippet end testcodeSqli012Safe
