fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

fn build_filter_from_json(input: &str) -> String {
    input.to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let q = req.param("filter");
    let doc = build_filter_from_json(&q);
    let result = mongo_find("products", &doc);
    super::shared::BenchmarkResponse::ok(&result)
}
