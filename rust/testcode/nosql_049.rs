//! CWE-943: Dead-code TN — tainted filter variable is built and then unconditionally replaced before use.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql049
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // user_filter is constructed from tainted input but then immediately overwritten.
    // The overwrite happens unconditionally so the tainted value can never reach mongo_find.
    let mut user_filter = format!("{{\"name\": \"{}\"}}", req.param("name"));
    user_filter = "{\"verified\":true}".to_string();
    let result = mongo_find("accounts", &user_filter); // vuln-code-snippet target-line testcodeNosql049
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql049
