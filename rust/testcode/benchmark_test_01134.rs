pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let sql = format!("SELECT id, email FROM users WHERE username = '{}'", username);
    super::shared::BenchmarkResponse::ok(&format!("Query: {}", sql))
}
