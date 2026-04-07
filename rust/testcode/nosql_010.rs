//! CWE-943: User-supplied status value interpolated into MongoDB session filter.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let status = req.param("status");
    // Attacker can inject {"$exists": true} to return all sessions regardless of status.
    let filter = format!("{{\"active\": \"{}\"}}", status);
    let result = mongo_find("sessions", &filter); // vuln-code-snippet target-line testcodeNosql010
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql010
