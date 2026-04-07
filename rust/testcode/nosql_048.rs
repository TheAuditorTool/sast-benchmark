//! CWE-943: Dead-code TN — compile-time impossible condition ("release" == "debug") guards tainted path.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql048
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // The string literals "release" and "debug" are never equal — the tainted branch is unreachable.
    let filter = if "release" == "debug" {
        format!("{{\"q\": \"{}\"}}", req.param("q"))
    } else {
        "{\"published\":true}".to_string()
    };
    let result = mongo_find("posts", &filter); // vuln-code-snippet target-line testcodeNosql048
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql048
