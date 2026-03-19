//! SQL Injection True Negative — CWE-89
//! Query with OFFSET from validated u32. Type system prevents SQL injection.
//! Non-numeric input fails parse and is rejected.

// vuln-code-snippet start testcodeSqli013Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.param("offset");

    // SAFE: Typed parse to u32 — only valid unsigned integers pass
    let offset: u32 = match raw.parse::<u32>() { // vuln-code-snippet safe-line testcodeSqli013Safe
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid offset"),
    };

    let query = format!("SELECT * FROM items LIMIT 20 OFFSET {}", offset);
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
// vuln-code-snippet end testcodeSqli013Safe
