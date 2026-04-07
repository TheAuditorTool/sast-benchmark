pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let term = req.param("search");

    let _query = "SELECT * FROM t WHERE name LIKE ?";
    let _bound = format!("%{}%", term);

    super::shared::BenchmarkResponse::ok(&format!("Parameterized LIKE for: {}", term))
}
