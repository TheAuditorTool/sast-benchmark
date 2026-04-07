pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.param("user_id");
    let user_id: u64 = match raw.parse::<u64>() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid user_id"),
    };
    let sql = format!("SELECT * FROM users WHERE id = {}", user_id);
    super::shared::BenchmarkResponse::ok(&sql)
}
