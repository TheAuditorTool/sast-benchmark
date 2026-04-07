pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");

    let query = format!("SELECT * FROM users WHERE id = {}", id);

    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
