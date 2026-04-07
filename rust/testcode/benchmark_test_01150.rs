pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let ids = req.param("ids");
    let sql = format!("SELECT * FROM notifications WHERE user_id IN ({})", ids);
    super::shared::BenchmarkResponse::ok(&sql)
}
