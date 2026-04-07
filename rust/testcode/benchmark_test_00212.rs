fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("username");
    if val.is_empty() {
        return super::shared::BenchmarkResponse::bad_request("username required");
    }
    let filter = format!("{{\"username\": \"{}\"}}", val);
    let result = mongo_find("users", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
