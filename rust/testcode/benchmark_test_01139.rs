fn build_filter(field: &str, value: &str) -> String {
    format!("{} = '{}'", field, value)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let col = req.param("col");
    let val = req.param("val");
    let filter = build_filter(&col, &val);
    let sql = format!("SELECT * FROM products WHERE {}", filter);
    super::shared::BenchmarkResponse::ok(&sql)
}
