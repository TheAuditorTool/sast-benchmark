fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

fn parse_as_json_value(input: &str) -> String {
    input.to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let filter = parse_as_json_value(&body);
    let result = mongo_find("users", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
