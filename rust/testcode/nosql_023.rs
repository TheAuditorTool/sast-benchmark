//! CWE-943: Space stripping fails to prevent operator injection — {"$ne":""} contains no spaces.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // Removing spaces is not a security control — MongoDB operators need no spaces.
    let val = req.param("f").replace(' ', "");
    let result = mongo_find("users", &format!("{{\"field\": \"{}\"}}", val)); // vuln-code-snippet target-line testcodeNosql023
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql023
