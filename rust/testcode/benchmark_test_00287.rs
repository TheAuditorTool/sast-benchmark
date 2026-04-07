fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("q");
    if val.len() >= 50 {
        return super::shared::BenchmarkResponse::bad_request("query too long");
    }
    let result = mongo_find("items", &format!("{{\"name\": \"{}\"}}", val));
    super::shared::BenchmarkResponse::ok(&result)
}
