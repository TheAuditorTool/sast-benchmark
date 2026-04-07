pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw_id = req.header("X-Resource-Id");
    let id: i64 = match raw_id.parse::<i64>() {
        Ok(v) if v > 0 => v,
        _ => return super::shared::BenchmarkResponse::bad_request("Invalid resource id"),
    };
    let sql = format!("SELECT * FROM resources WHERE id = {}", id);
    super::shared::BenchmarkResponse::ok(&sql)
}
