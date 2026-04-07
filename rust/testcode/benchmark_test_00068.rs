struct SearchQuery {
    q: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let query = SearchQuery { q: req.param("q") };

    let sql = format!("SELECT * FROM items WHERE name LIKE '%{}%'", query.q);

    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", sql))
}
