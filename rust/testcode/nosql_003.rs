//! CWE-943: User-controlled JSON filter parameter passed directly to MongoDB find.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

fn build_filter_from_json(input: &str) -> String {
    // Simulates: serde_json::from_str::<serde_json::Value>(input)
    //   then bson::to_document(&val).unwrap()
    // No validation of operator keys — any JSON object becomes the query document.
    input.to_string()
}

// vuln-code-snippet start testcodeNosql003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let q = req.param("filter");
    let doc = build_filter_from_json(&q);
    let result = mongo_find("products", &doc); // vuln-code-snippet target-line testcodeNosql003
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql003
