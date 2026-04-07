//! CWE-89: Taint from struct field reaches LIKE query via format!().

// vuln-code-snippet start testcodeSqli015
struct SearchQuery {
    q: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let query = SearchQuery { q: req.param("q") };

    let sql = format!("SELECT * FROM items WHERE name LIKE '%{}%'", query.q); // vuln-code-snippet target-line testcodeSqli015

    // In production: sqlx::query(&sql).fetch_all(pool).await
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", sql))
}
// vuln-code-snippet end testcodeSqli015
