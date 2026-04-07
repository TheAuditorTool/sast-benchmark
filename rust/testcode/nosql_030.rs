//! CWE-943: Safe status lookup — explicit Bson::String type prevents operator injection.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

fn build_safe_filter(field: &str, value: &str) -> String {
    // Simulates: doc! { field: Bson::String(value.to_string()) }
    // Explicit Bson::String prevents operator injection ($ne, $gt, etc. become literal strings).
    format!("{{\"{}\":{{\"$eq\":\"{}\"}}}}", field, value.replace('"', "\\\""))
}

// vuln-code-snippet start testcodeNosql030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let status = req.param("status");
    let filter = build_safe_filter("active", &status);
    let result = mongo_find("sessions", &filter); // vuln-code-snippet target-line testcodeNosql030
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql030
