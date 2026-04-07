pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    let balance = get_balance();
    if balance >= amount {
        set_balance(balance - amount);
        super::shared::BenchmarkResponse::ok("Transfer complete")
    } else {
        super::shared::BenchmarkResponse::bad_request("Insufficient funds")
    }
}

fn get_balance() -> i64 { 1000 }
fn set_balance(_bal: i64) {}
