fn db_get_invoice(id: &str) -> String {
    format!("invoice_data_for_{}", id)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("invoice_id");
    let invoice = db_get_invoice(&id);
    super::shared::BenchmarkResponse::ok(&invoice)
}
