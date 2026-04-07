fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut user_filter = format!("{{\"name\": \"{}\"}}", req.param("name"));
    user_filter = "{\"verified\":true}".to_string();
    let result = mongo_find("accounts", &user_filter);
    super::shared::BenchmarkResponse::ok(&result)
}
