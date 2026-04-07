//! CWE-943: User input wrapped in $or operator via format! chain — injection inside array element.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_filter = req.param("f");
    // If user_filter is {"$where":"function(){return true}"} the $or wrapping doesn't prevent it.
    let query = format!("{{\"$or\": [{}]}}", user_filter);
    let result = mongo_find("data", &query); // vuln-code-snippet target-line testcodeNosql014
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql014
