fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

struct UserSearch {
    email: String,
    role: String,
}

fn parse_user_search(body: &str) -> Result<UserSearch, String> {
    if body.contains('$') {
        return Err("Invalid search parameters".to_string());
    }
    Ok(UserSearch {
        email: body.to_string(),
        role: String::new(),
    })
}

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
    let result = mongo_find("users", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
