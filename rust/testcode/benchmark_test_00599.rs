fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

fn build_filter(q: &str) -> String {
    format!("{{\"q\": \"{}\"}}", q)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut m = std::collections::HashMap::new();
    m.insert("user_filter", build_filter(&req.param("q")));
    m.insert("safe_filter", "{\"active\":true}".to_string());
    let filter = m.get("safe_filter").map(String::as_str).unwrap_or("{\"active\":true}");
    let result = mongo_find("users", filter);
    super::shared::BenchmarkResponse::ok(&result)
}
