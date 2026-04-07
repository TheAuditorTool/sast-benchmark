fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let status = req.param("status");
    let filter = format!("{{\"active\": \"{}\"}}", status);
    let result = mongo_find("sessions", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
