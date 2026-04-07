//! CWE-943: User-supplied email address interpolated into MongoDB filter without sanitization.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    // A malicious email value can escape the string context and inject operators.
    let filter = format!("{{\"email\": \"{}\"}}", email);
    let result = mongo_find("subscribers", &filter); // vuln-code-snippet target-line testcodeNosql008
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql008
