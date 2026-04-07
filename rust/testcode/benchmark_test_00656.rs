fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _label = req.param("label");
    let result = mongo_find("items", "{\"available\":true}");
    super::shared::BenchmarkResponse::ok(&result)
}
