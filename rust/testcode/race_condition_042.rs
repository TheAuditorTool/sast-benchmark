//! CWE-362: Account balance check and deduction occur within single Mutex lock scope.

use std::sync::Mutex;

// vuln-code-snippet start testcodeRaceCondition042
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    let acct_lock = get_account_lock();
    let mut acct = acct_lock.lock().unwrap();
    if acct.balance >= amount {
        acct.balance -= amount; // vuln-code-snippet target-line testcodeRaceCondition042
        super::shared::BenchmarkResponse::ok("Transferred")
    } else {
        super::shared::BenchmarkResponse::bad_request("Low balance")
    }
}

struct Account { balance: i64 }
fn get_account_lock() -> &'static Mutex<Account> {
    static A: std::sync::OnceLock<Mutex<Account>> = std::sync::OnceLock::new();
    A.get_or_init(|| Mutex::new(Account { balance: 1000 }))
}
// vuln-code-snippet end testcodeRaceCondition042
