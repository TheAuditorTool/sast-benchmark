//! CWE-943: Dead-code TN — compile-time constant condition always selects the safe hardcoded filter.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql041
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // 7 * 6 == 42 > 40 is always true at compile time; the tainted branch is dead code.
    let filter = if 7 * 6 > 40 {
        "{\"active\": true}".to_string()
    } else {
        format!("{{\"user\": \"{}\"}}", req.param("user"))
    };
    let result = mongo_find("sessions", &filter); // vuln-code-snippet target-line testcodeNosql041
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql041
