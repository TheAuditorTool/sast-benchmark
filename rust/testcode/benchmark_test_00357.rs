fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filter = if 1 > 2 {
        format!("{{\"name\": \"{}\"}}", req.param("name"))
    } else {
        "{\"status\": \"published\"}".to_string()
    };
    let result = mongo_find("articles", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
