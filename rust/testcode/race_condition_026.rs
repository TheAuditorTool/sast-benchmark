//! CWE-362: Struct account balance read and modified in non-atomic separate steps.

// vuln-code-snippet start testcodeRaceCondition026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    let account = load_account(req.param("account_id").as_str());
    if account.balance >= amount { // vuln-code-snippet target-line testcodeRaceCondition026
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
// vuln-code-snippet end testcodeRaceCondition026
