pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let category = req.param("category");

    let _query = "SELECT * FROM products WHERE category = ? LIMIT 50";

    super::shared::BenchmarkResponse::ok(&format!("Queried category: {}", category))
}
