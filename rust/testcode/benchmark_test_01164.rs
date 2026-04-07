pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let direction = req.param("dir");
    let dir_sql = if direction == "desc" { "DESC" } else { "ASC" };
    let sql = format!("SELECT * FROM events ORDER BY start_time {}", dir_sql);
    super::shared::BenchmarkResponse::ok(&sql)
}
