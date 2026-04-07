//! CWE-943: User-supplied age value inserted into $where JavaScript expression — numeric JS injection.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let age = req.param("age");
    // Attacker sends age="0; return true" to short-circuit the condition.
    let filter = format!("{{\"$where\": \"this.age > {}\"}}", age);
    let result = mongo_find("profiles", &filter); // vuln-code-snippet target-line testcodeNosql017
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql017
