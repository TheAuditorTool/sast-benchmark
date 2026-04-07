//! CWE-943: Safe query — user input only selects a pre-defined query constant, never enters the filter.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql046
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // User input acts as a selector key only; the actual filter strings are all hardcoded.
    let filter = match req.param("q_type").as_str() {
        "active" => "{\"active\":true}",
        "admin" => "{\"role\":\"admin\"}",
        "verified" => "{\"verified\":true}",
        _ => return super::shared::BenchmarkResponse::bad_request("unknown query type"),
    };
    let result = mongo_find("users", filter); // vuln-code-snippet target-line testcodeNosql046
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql046
