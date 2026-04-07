pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let sql = "SELECT id, name, price FROM products WHERE featured = 1 ORDER BY created_at DESC LIMIT 20";
    super::shared::BenchmarkResponse::ok(sql)
}
