fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let field_name = match req.param("sort").as_str() {
        "name" => "name",
        "date" => "date",
        "price" => "price",
        _ => return super::shared::BenchmarkResponse::bad_request("invalid sort field"),
    };
    let filter = format!("{{\"{}\":{{\"$exists\":true}}}}", field_name);
    let result = mongo_find("products", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
