fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

fn build_safe_filter(field: &str, value: &str) -> String {
    format!("{{\"{}\":{{\"$eq\":\"{}\"}}}}", field, value.replace('"', "\\\""))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    let filter = build_safe_filter("email", &email);
    let result = mongo_find("subscribers", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
