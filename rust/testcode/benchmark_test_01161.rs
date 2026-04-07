pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw_min = req.param("min_price");
    let raw_max = req.param("max_price");
    let min: f64 = raw_min.parse().unwrap_or(0.0).max(0.0);
    let max: f64 = raw_max.parse().unwrap_or(10000.0).min(99999.0);
    let sql = format!("SELECT * FROM products WHERE price BETWEEN {} AND {}", min, max);
    super::shared::BenchmarkResponse::ok(&sql)
}
