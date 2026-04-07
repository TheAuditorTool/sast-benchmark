pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("name");

    let escaped = input.replace('\'', "''");

    let sql = format!("SELECT * FROM t WHERE name LIKE '{}'", escaped);

    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", sql))
}
