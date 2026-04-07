//! CWE-943: User-supplied username string interpolated directly into JSON filter via format!.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    // Attacker sends username={"$gt":""} which closes the string and injects an operator.
    let filter = format!("{{\"username\": \"{}\"}}", username);
    let result = mongo_find("users", &filter); // vuln-code-snippet target-line testcodeNosql006
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql006
