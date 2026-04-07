fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

fn build_query_from_params(field: &str, value: &str) -> String {
    format!("{{\"{}\": {}}}", field, value)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filter = build_query_from_params(&req.param("field"), &req.param("value"));
    let result = mongo_find("records", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
