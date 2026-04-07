fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filter = if "release" == "debug" {
        format!("{{\"q\": \"{}\"}}", req.param("q"))
    } else {
        "{\"published\":true}".to_string()
    };
    let result = mongo_find("posts", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
