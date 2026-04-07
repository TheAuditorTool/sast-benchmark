fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("search");
    if val.contains('$') {
        return super::shared::BenchmarkResponse::bad_request("invalid characters in search");
    }
    let filter = format!("{{\"title\":{{\"$eq\":\"{}\"}}}}", val.replace('"', "\\\""));
    let result = mongo_find("articles", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
