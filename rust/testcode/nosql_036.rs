//! CWE-943: Safe query — JSON object shape rejected before use, only plain strings accepted.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("q");
    if val.starts_with('{') || val.starts_with('[') {
        return super::shared::BenchmarkResponse::bad_request("structured input not allowed");
    }
    let filter = format!("{{\"name\":{{\"$eq\":\"{}\"}}}}", val.replace('"', "\\\""));
    let result = mongo_find("catalog", &filter); // vuln-code-snippet target-line testcodeNosql036
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql036
