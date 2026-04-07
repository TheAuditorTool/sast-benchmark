pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let price: f64 = req.param("price").parse().unwrap_or(f64::NAN);
    if !price.is_finite() || price <= 0.0 {
        return super::shared::BenchmarkResponse::bad_request("Invalid price");
    }
    let result = calculate_total(price);
    super::shared::BenchmarkResponse::ok(&format!("total={}", result))
}

fn calculate_total(p: f64) -> f64 { p * 1.1 }
