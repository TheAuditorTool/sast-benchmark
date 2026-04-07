fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("email");
    if !val.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '@' || c == '.') {
        return super::shared::BenchmarkResponse::bad_request("invalid email format");
    }
    let filter = format!("{{\"email\":{{\"$eq\":\"{}\"}}}}", val);
    let result = mongo_find("users", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
