fn mongo_find_one(collection: &str, filter: &str) -> Option<String> {
    Some(format!("Document from {} matching: {}", collection, filter))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let filter = format!("{{\"_id\": \"{}\"}}", id);
    let result = mongo_find_one("docs", &filter);
    match result {
        Some(doc) => super::shared::BenchmarkResponse::ok(&doc),
        None => super::shared::BenchmarkResponse::error("not found"),
    }
}
