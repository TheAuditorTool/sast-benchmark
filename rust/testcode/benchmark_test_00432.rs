fn mongo_find_one(collection: &str, filter: &str) -> Option<String> {
    Some(format!("Document from {} matching: {}", collection, filter))
}

fn build_safe_filter(field: &str, value: &str) -> String {
    format!("{{\"{}\":{{\"$eq\":\"{}\"}}}}", field, value.replace('"', "\\\""))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let filter = build_safe_filter("_id", &id);
    let result = mongo_find_one("docs", &filter);
    match result {
        Some(doc) => super::shared::BenchmarkResponse::ok(&doc),
        None => super::shared::BenchmarkResponse::error("not found"),
    }
}
