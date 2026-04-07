//! CWE-943: Safe query — $ operator characters rejected, preventing all MongoDB operator injection.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql037
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("search");
    if val.contains('$') {
        return super::shared::BenchmarkResponse::bad_request("invalid characters in search");
    }
    let filter = format!("{{\"title\":{{\"$eq\":\"{}\"}}}}", val.replace('"', "\\\""));
    let result = mongo_find("articles", &filter); // vuln-code-snippet target-line testcodeNosql037
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql037
