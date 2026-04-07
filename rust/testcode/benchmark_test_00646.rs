fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("q");
    if val.contains("$gt") {
        return super::shared::BenchmarkResponse::bad_request("operator not allowed");
    }
    let result = mongo_find("coll", &format!("{{\"q\": \"{}\"}}", val));
    super::shared::BenchmarkResponse::ok(&result)
}
