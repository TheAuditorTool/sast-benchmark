pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_search = req.param("search");

    let query = format!("SELECT * FROM t WHERE name LIKE '%{}%'", user_search);

    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
