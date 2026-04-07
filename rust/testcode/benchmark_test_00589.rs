fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    let filter = format!("{{\"$where\": \"this.username == '{}'\"}}", user);
    let result = mongo_find("accounts", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
