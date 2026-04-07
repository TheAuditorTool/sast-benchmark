fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    let filter = format!("{{\"email\": \"{}\"}}", email);
    let result = mongo_find("subscribers", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
