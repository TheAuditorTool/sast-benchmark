fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

fn build_safe_filter(field: &str, value: &str) -> String {
    format!("{{\"{}\":{{\"$eq\":\"{}\"}}}}", field, value.replace('"', "\\\""))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let filter = build_safe_filter("username", &username);
    let result = mongo_find("users", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
