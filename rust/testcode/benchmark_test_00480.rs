fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("v");
    if !val.is_ascii() {
        return super::shared::BenchmarkResponse::bad_request("ascii only");
    }
    let result = mongo_find("docs", &format!("{{\"v\": \"{}\"}}", val));
    super::shared::BenchmarkResponse::ok(&result)
}
