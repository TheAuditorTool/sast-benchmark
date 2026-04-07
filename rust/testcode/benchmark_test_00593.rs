fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filter = match req.param("q_type").as_str() {
        "active" => "{\"active\":true}",
        "admin" => "{\"role\":\"admin\"}",
        "verified" => "{\"verified\":true}",
        _ => return super::shared::BenchmarkResponse::bad_request("unknown query type"),
    };
    let result = mongo_find("users", filter);
    super::shared::BenchmarkResponse::ok(&result)
}
