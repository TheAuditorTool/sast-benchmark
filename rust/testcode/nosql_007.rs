//! CWE-943: User-supplied document ID interpolated into MongoDB filter string.

fn mongo_find_one(collection: &str, filter: &str) -> Option<String> {
    // In production: mongodb::Collection::find_one(filter, None).await
    Some(format!("Document from {} matching: {}", collection, filter))
}

// vuln-code-snippet start testcodeNosql007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    // Attacker can break out of the string and inject operators.
    let filter = format!("{{\"_id\": \"{}\"}}", id);
    let result = mongo_find_one("docs", &filter); // vuln-code-snippet target-line testcodeNosql007
    match result {
        Some(doc) => super::shared::BenchmarkResponse::ok(&doc),
        None => super::shared::BenchmarkResponse::error("not found"),
    }
}
// vuln-code-snippet end testcodeNosql007
