pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let col = req.param("sort_col");
    let allowed = ["name", "price", "created_at", "rating"];
    if !allowed.contains(&col.as_str()) {
        return super::shared::BenchmarkResponse::bad_request("Invalid sort column");
    }
    let sql = format!("SELECT * FROM products ORDER BY {}", col);
    super::shared::BenchmarkResponse::ok(&sql)
}
