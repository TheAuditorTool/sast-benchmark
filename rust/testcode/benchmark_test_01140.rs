pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let category = req.param("category");
    let sort = req.param("sort");
    let sql = format!(
        "SELECT id, name, price FROM products WHERE category = '{}' ORDER BY {}",
        category, sort
    );
    super::shared::BenchmarkResponse::ok(&sql)
}
