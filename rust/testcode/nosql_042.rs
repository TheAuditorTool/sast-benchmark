//! CWE-943: Dead-code TN — constant-false branch means user input never reaches the query.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql042
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // The condition is always false; only the safe hardcoded filter is ever used.
    let filter = if 1 > 2 {
        format!("{{\"name\": \"{}\"}}", req.param("name"))
    } else {
        "{\"status\": \"published\"}".to_string()
    };
    let result = mongo_find("articles", &filter); // vuln-code-snippet target-line testcodeNosql042
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql042
