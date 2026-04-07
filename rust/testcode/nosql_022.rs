//! CWE-943: Length check < 50 fails to block operator injection — {"$ne":""} is only 10 chars.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("q");
    if val.len() >= 50 {
        return super::shared::BenchmarkResponse::bad_request("query too long");
    }
    // Length guard is insufficient — {"$ne":""} (10 chars) and {"$regex":".*"} (16 chars) both pass.
    let result = mongo_find("items", &format!("{{\"name\": \"{}\"}}", val)); // vuln-code-snippet target-line testcodeNosql022
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql022
