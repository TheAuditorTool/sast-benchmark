//! CWE-943: ASCII-only check fails to prevent operator injection — {"$ne":""} is entirely ASCII.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("v");
    if !val.is_ascii() {
        return super::shared::BenchmarkResponse::bad_request("ascii only");
    }
    // is_ascii() only rejects non-ASCII bytes — all MongoDB operators are ASCII.
    let result = mongo_find("docs", &format!("{{\"v\": \"{}\"}}", val)); // vuln-code-snippet target-line testcodeNosql024
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql024
