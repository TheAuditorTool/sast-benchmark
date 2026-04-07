fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filter = if 7 * 6 > 40 {
        "{\"active\": true}".to_string()
    } else {
        format!("{{\"user\": \"{}\"}}", req.param("user"))
    };
    let result = mongo_find("sessions", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
