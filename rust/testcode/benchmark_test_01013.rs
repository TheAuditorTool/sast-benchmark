pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.param("id");

    let id: i64 = match raw.parse::<i64>() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid id"),
    };

    let query = format!("SELECT * FROM users WHERE id = {}", id);
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
