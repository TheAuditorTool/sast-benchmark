//! CWE-943: Safe document ID lookup — explicit Bson::String type prevents operator injection.

fn mongo_find_one(collection: &str, filter: &str) -> Option<String> {
    // In production: mongodb::Collection::find_one(filter, None).await
    Some(format!("Document from {} matching: {}", collection, filter))
}

fn build_safe_filter(field: &str, value: &str) -> String {
    // Simulates: doc! { field: Bson::String(value.to_string()) }
    // Explicit Bson::String prevents operator injection ($ne, $gt, etc. become literal strings).
    format!("{{\"{}\":{{\"$eq\":\"{}\"}}}}", field, value.replace('"', "\\\""))
}

// vuln-code-snippet start testcodeNosql028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let filter = build_safe_filter("_id", &id);
    let result = mongo_find_one("docs", &filter); // vuln-code-snippet target-line testcodeNosql028
    match result {
        Some(doc) => super::shared::BenchmarkResponse::ok(&doc),
        None => super::shared::BenchmarkResponse::error("not found"),
    }
}
// vuln-code-snippet end testcodeNosql028
