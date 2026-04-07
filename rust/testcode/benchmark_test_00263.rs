fn transfer_funds(to: &str, amount: &str) -> bool {
    let _ = (to, amount);
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let to = req.param("to");
    let amount = req.param("amount");
    let result = transfer_funds(&to, &amount);
    if result {
        super::shared::BenchmarkResponse::ok("transfer complete")
    } else {
        super::shared::BenchmarkResponse::error("transfer failed")
    }
}
