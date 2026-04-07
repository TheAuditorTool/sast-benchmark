fn transfer_funds(to: &str, amount: &str) -> bool {
    let _ = (to, amount);
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.header("Origin") != "https://app.example.com" {
        return super::shared::BenchmarkResponse::forbidden("invalid origin");
    }
    let to = req.param("to");
    let amount = req.param("amount");
    let result = transfer_funds(&to, &amount);
    if result {
        super::shared::BenchmarkResponse::ok("transfer complete")
    } else {
        super::shared::BenchmarkResponse::error("transfer failed")
    }
}
