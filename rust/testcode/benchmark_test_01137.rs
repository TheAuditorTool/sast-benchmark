pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let order_id = req.header("X-Order-Id");
    let sql = format!("SELECT * FROM orders WHERE order_id = {}", order_id);
    super::shared::BenchmarkResponse::ok(&sql)
}
