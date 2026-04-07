pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let query = format!("SELECT * FROM users WHERE id = {}", id);
    let err_msg = format!("SQL error in query '{}': syntax error near '{}'", query, id);
    super::shared::BenchmarkResponse::error(&err_msg)
}
