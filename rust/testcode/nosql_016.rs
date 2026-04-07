//! CWE-943: User input injected into MongoDB $where JavaScript clause — enables JS code execution.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    // Attacker sends user="' || '1'=='1" to make the JS condition always true.
    let filter = format!("{{\"$where\": \"this.username == '{}'\"}}", user);
    let result = mongo_find("accounts", &filter); // vuln-code-snippet target-line testcodeNosql016
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql016
