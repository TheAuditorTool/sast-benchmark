//! CWE-943: Dead-code TN — tainted filter built then immediately overwritten with safe constant.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql043
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // The tainted value is unconditionally overwritten before the query — never reaches mongo_find.
    let mut filter = format!("{{\"user\": \"{}\"}}", req.param("u"));
    filter = "{\"status\":\"active\"}".to_string();
    let result = mongo_find("sessions", &filter); // vuln-code-snippet target-line testcodeNosql043
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql043
