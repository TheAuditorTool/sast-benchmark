fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let role = req.param("role");
    let filter = format!("{{\"role\": \"{}\"}}", role);
    let result = mongo_find("users", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
