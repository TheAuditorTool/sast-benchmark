//! CWE-943: Safe role lookup — explicit Bson::String type prevents operator injection.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

fn build_safe_filter(field: &str, value: &str) -> String {
    // Simulates: doc! { field: Bson::String(value.to_string()) }
    // Explicit Bson::String prevents operator injection ($ne, $gt, etc. become literal strings).
    format!("{{\"{}\":{{\"$eq\":\"{}\"}}}}", field, value.replace('"', "\\\""))
}

// vuln-code-snippet start testcodeNosql029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let role = req.param("role");
    let filter = build_safe_filter("role", &role);
    let result = mongo_find("users", &filter); // vuln-code-snippet target-line testcodeNosql029
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql029
