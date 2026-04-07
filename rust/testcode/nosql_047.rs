//! CWE-943: Safe query — user input used only as a display label, filter is always hardcoded.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql047
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // _label is read from user input but never reaches the database query.
    let _label = req.param("label");
    let result = mongo_find("items", "{\"available\":true}"); // vuln-code-snippet target-line testcodeNosql047
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql047
