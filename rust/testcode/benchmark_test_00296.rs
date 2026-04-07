pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_col = req.param("sort");

    let col = match user_col.as_str() {
        "name" | "date" | "id" => user_col.as_str(),
        _ => "id",
    };

    let query = format!("SELECT * FROM t ORDER BY {}", col);
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
