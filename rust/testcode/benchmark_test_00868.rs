const DB_PASSWORD: &str = "P@ssw0rd2024!Prod";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let db_name = req.param("db");
    let conn_str = format!("postgres://app:{}@db:5432/{}", DB_PASSWORD, db_name);
    let result = format!("Connected using: {}", conn_str);
    super::shared::BenchmarkResponse::ok(&result)
}
