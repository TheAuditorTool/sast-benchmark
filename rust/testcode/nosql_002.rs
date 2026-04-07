//! CWE-943: Login bypass — user-supplied JSON with {"$ne": ""} operators authenticates without valid credentials.

fn mongo_find_one(collection: &str, filter: &str) -> Option<String> {
    // In production: mongodb::Collection::find_one(filter, None).await
    Some(format!("Document from {} matching: {}", collection, filter))
}

fn parse_as_json_value(input: &str) -> String {
    // Simulates: serde_json::from_str::<serde_json::Value>(input)
    // Untyped deserialization — operator keys like "$ne" are preserved as-is.
    input.to_string()
}

// vuln-code-snippet start testcodeNosql002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // Attacker sends: {"username": {"$ne": ""}, "password": {"$ne": ""}}
    // Deserialized as Value, operators reach the database engine unchanged.
    let filter = parse_as_json_value(&req.param("query"));
    let result = mongo_find_one("accounts", &filter); // vuln-code-snippet target-line testcodeNosql002
    match result {
        Some(doc) => super::shared::BenchmarkResponse::ok(&doc),
        None => super::shared::BenchmarkResponse::forbidden("invalid credentials"),
    }
}
// vuln-code-snippet end testcodeNosql002
