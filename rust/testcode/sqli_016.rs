//! CWE-89: Single-quote escaping applied but LIKE wildcards (% and _) still injectable.

// vuln-code-snippet start testcodeSqli016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("name");

    // Partial sanitization: escapes single quotes but leaves LIKE metacharacters intact
    let escaped = input.replace('\'', "''");

    let sql = format!("SELECT * FROM t WHERE name LIKE '{}'", escaped); // vuln-code-snippet target-line testcodeSqli016

    // In production: sqlx::query(&sql).fetch_all(pool).await
    // Attacker can inject: name=% to match all rows, or name=_ for single-char wildcard
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", sql))
}
// vuln-code-snippet end testcodeSqli016
