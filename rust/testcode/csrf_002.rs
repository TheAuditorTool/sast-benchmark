//! CWE-352: State-mutating funds transfer with no CSRF token check.

fn transfer_funds(to: &str, amount: &str) -> bool {
    let _ = (to, amount);
    true
}

// vuln-code-snippet start testcodeCsrf002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let to = req.param("to");
    let amount = req.param("amount");
    let result = transfer_funds(&to, &amount); // vuln-code-snippet target-line testcodeCsrf002
    if result {
        super::shared::BenchmarkResponse::ok("transfer complete")
    } else {
        super::shared::BenchmarkResponse::error("transfer failed")
    }
}
// vuln-code-snippet end testcodeCsrf002
