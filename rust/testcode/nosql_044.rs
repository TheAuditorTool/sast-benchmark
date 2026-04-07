//! CWE-943: Dead-code TN — HashMap holds tainted filter but query reads from the safe key.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

fn build_filter(q: &str) -> String {
    format!("{{\"q\": \"{}\"}}", q)
}

// vuln-code-snippet start testcodeNosql044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut m = std::collections::HashMap::new();
    m.insert("user_filter", build_filter(&req.param("q")));
    m.insert("safe_filter", "{\"active\":true}".to_string());
    // Only "safe_filter" is read — the tainted value stored under "user_filter" is never used.
    let filter = m.get("safe_filter").map(String::as_str).unwrap_or("{\"active\":true}");
    let result = mongo_find("users", filter); // vuln-code-snippet target-line testcodeNosql044
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql044
