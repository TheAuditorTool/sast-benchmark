fn db_get_order(order_id: &str) -> String {
    format!("order_details_for_{}", order_id)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let order_id = req.param("order_id");
    let order = db_get_order(&order_id);
    super::shared::BenchmarkResponse::ok(&order)
}
