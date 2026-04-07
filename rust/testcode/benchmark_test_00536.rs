fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

fn safe_query(_user_input: &str) -> &'static str {
    "{\"active\":true}"
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filter = safe_query(&req.param("q"));
    let result = mongo_find("records", filter);
    super::shared::BenchmarkResponse::ok(&result)
}
