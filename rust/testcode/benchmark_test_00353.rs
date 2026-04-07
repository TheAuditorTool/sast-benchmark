pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    let account = load_account(req.param("account_id").as_str());
    if account.balance >= amount {
        let new_bal = account.balance - amount;
        save_account_balance(req.param("account_id").as_str(), new_bal);
        super::shared::BenchmarkResponse::ok("Deducted")
    } else {
        super::shared::BenchmarkResponse::bad_request("Insufficient")
    }
}

struct Account { balance: i64 }
fn load_account(_id: &str) -> Account { Account { balance: 500 } }
fn save_account_balance(_id: &str, _bal: i64) {}
