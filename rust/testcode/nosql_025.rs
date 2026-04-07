//! CWE-943: Blocklist rejects $gt but allows $ne, $regex, $where — incomplete operator filtering.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("q");
    // Blocklist approach: only rejects $gt, leaving $ne, $regex, $where, $in, $exists operational.
    if val.contains("$gt") {
        return super::shared::BenchmarkResponse::bad_request("operator not allowed");
    }
    let result = mongo_find("coll", &format!("{{\"q\": \"{}\"}}", val)); // vuln-code-snippet target-line testcodeNosql025
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql025
