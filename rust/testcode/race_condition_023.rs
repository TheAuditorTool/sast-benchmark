//! CWE-362: Balance read and deduct are separate non-atomic operations, allowing overdraft race.

// vuln-code-snippet start testcodeRaceCondition023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    let balance = get_balance(); // vuln-code-snippet target-line testcodeRaceCondition023
    if balance >= amount {
        set_balance(balance - amount);
        super::shared::BenchmarkResponse::ok("Transfer complete")
    } else {
        super::shared::BenchmarkResponse::bad_request("Insufficient funds")
    }
}

fn get_balance() -> i64 { 1000 }
fn set_balance(_bal: i64) {}
// vuln-code-snippet end testcodeRaceCondition023
