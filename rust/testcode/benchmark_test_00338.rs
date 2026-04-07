const DB_URL: &str = "postgres://admin:s3cretpass@db:5432/app";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let table = req.param("table");
    let result = format!("Connected to {} for table {}", DB_URL, table);
    super::shared::BenchmarkResponse::ok(&result)
}
