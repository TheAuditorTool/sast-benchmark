fn mongo_find_one(collection: &str, filter: &str) -> Option<String> {
    Some(format!("Document from {} matching: {}", collection, filter))
}

fn parse_as_json_value(input: &str) -> String {
    input.to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filter = parse_as_json_value(&req.param("query"));
    let result = mongo_find_one("accounts", &filter);
    match result {
        Some(doc) => super::shared::BenchmarkResponse::ok(&doc),
        None => super::shared::BenchmarkResponse::forbidden("invalid credentials"),
    }
}
