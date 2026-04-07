fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let js_condition = req.param("condition");
    let filter = format!("{{\"$where\": \"{}\"}}", js_condition);
    let result = mongo_find("events", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
