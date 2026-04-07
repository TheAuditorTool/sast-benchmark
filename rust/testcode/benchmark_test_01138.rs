pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.body_str();
    let sql = format!("INSERT INTO messages (content) VALUES ('{}')", body);
    super::shared::BenchmarkResponse::ok(&format!("Inserted: {}", sql))
}
