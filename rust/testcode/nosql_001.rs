//! CWE-943: User-supplied JSON body deserialized to untyped Value and used as MongoDB filter.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

fn parse_as_json_value(input: &str) -> String {
    // Simulates: serde_json::from_str::<serde_json::Value>(input) then bson::to_document(&val)
    // Returns the raw input as the filter document — no type enforcement, no operator rejection.
    // {"$gt": ""} or {"$ne": null} pass through unchanged.
    input.to_string()
}

// vuln-code-snippet start testcodeNosql001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let filter = parse_as_json_value(&body);
    let result = mongo_find("users", &filter); // vuln-code-snippet target-line testcodeNosql001
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql001
