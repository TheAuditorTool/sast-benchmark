fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("f").replace(' ', "");
    let result = mongo_find("users", &format!("{{\"field\": \"{}\"}}", val));
    super::shared::BenchmarkResponse::ok(&result)
}
