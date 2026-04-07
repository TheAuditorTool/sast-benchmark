pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_col = req.param("sort");

    let query = format!("SELECT * FROM t ORDER BY {}", user_col);

    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
