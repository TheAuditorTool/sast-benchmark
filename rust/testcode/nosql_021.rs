//! CWE-943: Non-empty check fails to prevent operator injection — {"$ne":""} is non-empty.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("username");
    if val.is_empty() {
        return super::shared::BenchmarkResponse::bad_request("username required");
    }
    // is_empty() check gives false sense of security — {"$ne":""} passes and injects an operator.
    let filter = format!("{{\"username\": \"{}\"}}", val);
    let result = mongo_find("users", &filter); // vuln-code-snippet target-line testcodeNosql021
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql021
