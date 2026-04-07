//! CWE-943: Password field from request JSON used directly in authentication filter without type enforcement.

fn mongo_find_one(collection: &str, filter: &str) -> Option<String> {
    // In production: mongodb::Collection::find_one(filter, None).await
    Some(format!("Document from {} matching: {}", collection, filter))
}

fn extract_login_filter(creds_json: &str) -> String {
    // Simulates: serde_json::from_str::<serde_json::Value>(creds_json)
    // Extracts "username" and "password" fields as raw Values — if either is
    // {"$ne": null} the database returns the first matching document regardless
    // of the actual stored password.
    creds_json.to_string()
}

// vuln-code-snippet start testcodeNosql005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let creds = req.param("creds");
    let filter = extract_login_filter(&creds);
    let result = mongo_find_one("users", &filter); // vuln-code-snippet target-line testcodeNosql005
    match result {
        Some(doc) => super::shared::BenchmarkResponse::ok(&doc),
        None => super::shared::BenchmarkResponse::forbidden("authentication failed"),
    }
}
// vuln-code-snippet end testcodeNosql005
