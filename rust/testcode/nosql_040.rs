//! CWE-943: Safe sort field — user input selects from a hardcoded allowlist, never reaches the filter.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql040
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // User input only selects a pre-approved field name; arbitrary strings are rejected.
    let field_name = match req.param("sort").as_str() {
        "name" => "name",
        "date" => "date",
        "price" => "price",
        _ => return super::shared::BenchmarkResponse::bad_request("invalid sort field"),
    };
    let filter = format!("{{\"{}\":{{\"$exists\":true}}}}", field_name);
    let result = mongo_find("products", &filter); // vuln-code-snippet target-line testcodeNosql040
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql040
