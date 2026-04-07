//! CWE-943: Safe user search — typed UserSearch struct deserialization prevents operator injection.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

struct UserSearch {
    email: String,
    role: String,
}

fn parse_user_search(body: &str) -> Result<UserSearch, String> {
    // Simulates typed serde deserialization — String fields reject operator objects.
    if body.contains('$') {
        return Err("Invalid search parameters".to_string());
    }
    Ok(UserSearch {
        email: body.to_string(),
        role: String::new(),
    })
}

// vuln-code-snippet start testcodeNosql033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let us = match parse_user_search(&body) {
        Ok(s) => s,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e),
    };
    let filter = format!(
        "{{\"email\":{{\"$eq\":\"{}\"}},\"role\":{{\"$eq\":\"{}\"}}}}",
        us.email.replace('"', "\\\""),
        us.role.replace('"', "\\\"")
    );
    let result = mongo_find("users", &filter); // vuln-code-snippet target-line testcodeNosql033
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql033
