pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw_count = req.param("count");
    let count: u32 = match raw_count.parse::<u32>() {
        Ok(c) if c <= 50 => c,
        _ => return super::shared::BenchmarkResponse::bad_request("Count must be 0-50"),
    };
    let sql = format!("SELECT * FROM logs ORDER BY timestamp DESC LIMIT {}", count);
    super::shared::BenchmarkResponse::ok(&sql)
}
