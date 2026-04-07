//! CWE-943: Safe login — typed struct deserialization rejects operator objects in username/password.

fn mongo_find_one(collection: &str, filter: &str) -> Option<String> {
    // In production: mongodb::Collection::find_one(filter, None).await
    Some(format!("Document from {} matching: {}", collection, filter))
}

struct LoginRequest {
    username: String,
    password: String,
}

fn parse_login_request(body: &str) -> Result<LoginRequest, String> {
    // Simulates typed serde deserialization — field types enforce String, not Value.
    // {"$ne": ""} would fail to deserialize as a String field.
    if body.contains('$') {
        return Err("Invalid input".to_string());
    }
    Ok(LoginRequest {
        username: body.to_string(),
        password: String::new(),
    })
}

// vuln-code-snippet start testcodeNosql031
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let login = match parse_login_request(&body) {
        Ok(l) => l,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e),
    };
    let filter = format!(
        "{{\"username\":{{\"$eq\":\"{}\"}},\"password\":{{\"$eq\":\"{}\"}}}}",
        login.username.replace('"', "\\\""),
        login.password.replace('"', "\\\"")
    );
    let result = mongo_find_one("users", &filter); // vuln-code-snippet target-line testcodeNosql031
    match result {
        Some(doc) => super::shared::BenchmarkResponse::ok(&doc),
        None => super::shared::BenchmarkResponse::forbidden("invalid credentials"),
    }
}
// vuln-code-snippet end testcodeNosql031
