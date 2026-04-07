pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let tag = req.param("tag");
    let limit = req.param("limit");
    let sql = format!(
        "SELECT * FROM articles WHERE tags LIKE '%{}%' LIMIT {}",
        tag, limit
    );
    super::shared::BenchmarkResponse::ok(&sql)
}
