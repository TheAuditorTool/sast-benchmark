fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut filters: Vec<String> = Vec::new();
    filters.push("{\"active\":true}".to_string());
    filters.push(format!("{{\"user\": \"{}\"}}", req.param("u")));
    filters.pop();
    let filter = filters.get(0).map(String::as_str).unwrap_or("{\"active\":true}");
    let result = mongo_find("sessions", filter);
    super::shared::BenchmarkResponse::ok(&result)
}
