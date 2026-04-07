fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_filter = req.param("f");
    let query = format!("{{\"$or\": [{}]}}", user_filter);
    let result = mongo_find("data", &query);
    super::shared::BenchmarkResponse::ok(&result)
}
