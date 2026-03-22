//! CWE-89: ORDER BY from allowlist. Invalid input falls back to default.

// vuln-code-snippet start testcodeSqli005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_col = req.param("sort");

    let col = match user_col.as_str() { // vuln-code-snippet target-line testcodeSqli005
        "name" | "date" | "id" => user_col.as_str(),
        _ => "id",
    };

    let query = format!("SELECT * FROM t ORDER BY {}", col);
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
// vuln-code-snippet end testcodeSqli005
