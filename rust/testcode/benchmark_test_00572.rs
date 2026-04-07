fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("q");
    if val.starts_with('{') || val.starts_with('[') {
        return super::shared::BenchmarkResponse::bad_request("structured input not allowed");
    }
    let filter = format!("{{\"name\":{{\"$eq\":\"{}\"}}}}", val.replace('"', "\\\""));
    let result = mongo_find("catalog", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
