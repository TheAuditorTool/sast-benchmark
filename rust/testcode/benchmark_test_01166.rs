pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let flag = req.param("active");
    let active: bool = flag == "true" || flag == "1";
    let sql = format!("SELECT * FROM subscriptions WHERE active = {}", active as u8);
    super::shared::BenchmarkResponse::ok(&sql)
}
