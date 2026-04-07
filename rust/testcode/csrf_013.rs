//! CWE-352: CSRF token length check (==32) before funds transfer — correct length, wrong value passes.

fn transfer_funds(to: &str, amount: &str) -> bool {
    let _ = (to, amount);
    true
}

// vuln-code-snippet start testcodeCsrf013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let csrf = req.param("csrf");
    if csrf.len() != 32 {
        return super::shared::BenchmarkResponse::bad_request("invalid csrf token");
    }
    // Length matches but the actual value is never compared to the session token.
    let to = req.param("to");
    let amount = req.param("amount");
    let result = transfer_funds(&to, &amount); // vuln-code-snippet target-line testcodeCsrf013
    if result {
        super::shared::BenchmarkResponse::ok("transfer complete")
    } else {
        super::shared::BenchmarkResponse::error("transfer failed")
    }
}
// vuln-code-snippet end testcodeCsrf013
