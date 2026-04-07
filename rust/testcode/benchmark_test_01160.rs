pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let category = req.param("category");
    let allowed = ["electronics", "clothing", "books", "sports", "home"];
    if !allowed.contains(&category.as_str()) {
        return super::shared::BenchmarkResponse::bad_request("Unknown category");
    }
    let sql = format!("SELECT id, name, price FROM products WHERE category = '{}'", category);
    super::shared::BenchmarkResponse::ok(&sql)
}
