pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let table = req.param("table");
    let db_password = std::env::var("DB_PASSWORD").unwrap_or_default();
    let db_url = format!("postgres://admin:{}@db:5432/app", db_password);
    let result = format!("Connected for table {}", table);
    super::shared::BenchmarkResponse::ok(&result)
}
