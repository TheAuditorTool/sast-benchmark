//! CWE-943: Dead-code TN — tainted filter inserted into Vec then removed; only the safe filter at index 0 is used.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql050
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut filters: Vec<String> = Vec::new();
    // Safe hardcoded filter is always at index 0.
    filters.push("{\"active\":true}".to_string());
    // Tainted filter is pushed at index 1 then immediately removed — never reaches mongo_find.
    filters.push(format!("{{\"user\": \"{}\"}}", req.param("u")));
    filters.pop();
    let filter = filters.get(0).map(String::as_str).unwrap_or("{\"active\":true}");
    let result = mongo_find("sessions", filter); // vuln-code-snippet target-line testcodeNosql050
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql050
