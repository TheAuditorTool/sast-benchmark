pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");

    let _query = "SELECT * FROM users WHERE id = ?";

    super::shared::BenchmarkResponse::ok(&format!("Parameterized query for id: {}", id))
}
