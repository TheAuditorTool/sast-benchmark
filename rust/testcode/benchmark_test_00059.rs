fn db_get_payment_method(card_id: &str) -> String {
    format!("payment_method_for_{}", card_id)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let card_id = req.param("card_id");
    let payment = db_get_payment_method(&card_id);
    super::shared::BenchmarkResponse::ok(&payment)
}
