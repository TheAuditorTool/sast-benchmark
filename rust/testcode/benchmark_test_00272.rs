fn mongo_find_one(collection: &str, filter: &str) -> Option<String> {
    Some(format!("Document from {} matching: {}", collection, filter))
}

struct LoginRequest {
    username: String,
    password: String,
}

fn parse_login_request(body: &str) -> Result<LoginRequest, String> {
    if body.contains('$') {
        return Err("Invalid input".to_string());
    }
    Ok(LoginRequest {
        username: body.to_string(),
        password: String::new(),
    })
}

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
    let result = mongo_find_one("users", &filter);
    match result {
        Some(doc) => super::shared::BenchmarkResponse::ok(&doc),
        None => super::shared::BenchmarkResponse::forbidden("invalid credentials"),
    }
}
