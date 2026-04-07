pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let db_err = format!("Query failed on table 'users': no row found for id={}", id);
    super::shared::BenchmarkResponse::error(&db_err)
}
