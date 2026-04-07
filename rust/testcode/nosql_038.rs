//! CWE-943: Safe query — strict allowlist (alphanumeric, hyphen, @, .) blocks all operator characters.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("email");
    if !val.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '@' || c == '.') {
        return super::shared::BenchmarkResponse::bad_request("invalid email format");
    }
    let filter = format!("{{\"email\":{{\"$eq\":\"{}\"}}}}", val);
    let result = mongo_find("users", &filter); // vuln-code-snippet target-line testcodeNosql038
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql038
