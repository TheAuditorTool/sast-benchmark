pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let price_str = req.param("price");
    let price: f64 = price_str.parse().unwrap_or(0.0);
    let total = price * 1.08;
    super::shared::BenchmarkResponse::ok(&format!("Total: {:.2}", total))
}
