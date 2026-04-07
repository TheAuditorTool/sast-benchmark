//! CWE-943: Dead-code TN — helper function ignores its argument and returns a hardcoded safe filter.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

fn safe_query(_user_input: &str) -> &'static str {
    // User input is accepted as a parameter but intentionally ignored.
    // The function always returns a hardcoded filter — no taint can reach mongo_find.
    "{\"active\":true}"
}

// vuln-code-snippet start testcodeNosql045
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filter = safe_query(&req.param("q"));
    let result = mongo_find("records", filter); // vuln-code-snippet target-line testcodeNosql045
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql045
