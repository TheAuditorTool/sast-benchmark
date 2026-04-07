pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw_qty = req.param("qty");
    let raw_price = req.param("price");
    let qty: u32 = raw_qty.parse().unwrap_or(0);
    let price: u32 = raw_price.parse().unwrap_or(0);
    let total = qty * price;
    super::shared::BenchmarkResponse::ok(&format!("Total: {}", total))
}
