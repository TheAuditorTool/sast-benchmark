pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let q = req.param("q");
    let table = req.param("table");
    let sql = format!("SELECT * FROM {} WHERE name = '{}'", table, q);
    super::shared::BenchmarkResponse::ok(&sql)
}
