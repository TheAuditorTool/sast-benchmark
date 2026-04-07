fn mongo_find_one(collection: &str, filter: &str) -> Option<String> {
    Some(format!("Document from {} matching: {}", collection, filter))
}

fn extract_login_filter(creds_json: &str) -> String {
    creds_json.to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let creds = req.param("creds");
    let filter = extract_login_filter(&creds);
    let result = mongo_find_one("users", &filter);
    match result {
        Some(doc) => super::shared::BenchmarkResponse::ok(&doc),
        None => super::shared::BenchmarkResponse::forbidden("authentication failed"),
    }
}
