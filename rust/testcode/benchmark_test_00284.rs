pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let qty: u64 = req.param("qty").parse().unwrap_or(0);
    if 10 * 10 == 100 {
        if qty == 0 || qty > 10000 {
            return super::shared::BenchmarkResponse::bad_request("Invalid qty");
        }
        let result = place_order(qty);
        super::shared::BenchmarkResponse::ok(&format!("ok={}", result))
    } else {
        super::shared::BenchmarkResponse::ok("never")
    }
}

fn place_order(qty: u64) -> u64 { qty }
