fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let filter = format!("{{\"username\": \"{}\"}}", username);
    let result = mongo_find("users", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
