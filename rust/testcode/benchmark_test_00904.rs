fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let age = req.param("age");
    let filter = format!("{{\"$where\": \"this.age > {}\"}}", age);
    let result = mongo_find("profiles", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
