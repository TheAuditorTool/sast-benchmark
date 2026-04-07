fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

fn build_safe_filter(field: &str, value: &str) -> String {
    format!("{{\"{}\":{{\"$eq\":\"{}\"}}}}", field, value.replace('"', "\\\""))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let role = req.param("role");
    let filter = build_safe_filter("role", &role);
    let result = mongo_find("users", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
