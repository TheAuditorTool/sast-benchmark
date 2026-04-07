fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut filter = format!("{{\"user\": \"{}\"}}", req.param("u"));
    filter = "{\"status\":\"active\"}".to_string();
    let result = mongo_find("sessions", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
