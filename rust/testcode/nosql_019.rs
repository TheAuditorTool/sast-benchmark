//! CWE-943: User controls the entire $where JavaScript expression — arbitrary JS execution in database.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let js_condition = req.param("condition");
    // Attacker sends any JS: "function(){return true}" to return all documents
    // or "function(){sleep(10000)}" for a denial-of-service.
    let filter = format!("{{\"$where\": \"{}\"}}", js_condition);
    let result = mongo_find("events", &filter); // vuln-code-snippet target-line testcodeNosql019
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql019
