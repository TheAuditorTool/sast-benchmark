//! CWE-943: User-supplied role parameter interpolated into MongoDB filter string.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let role = req.param("role");
    // Attacker can inject {"$in": ["admin","superuser"]} to match elevated roles.
    let filter = format!("{{\"role\": \"{}\"}}", role);
    let result = mongo_find("users", &filter); // vuln-code-snippet target-line testcodeNosql009
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql009
