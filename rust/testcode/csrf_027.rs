//! CWE-352: Proper CSRF protection — constant-time token comparison before funds transfer.

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn transfer_funds(to: &str, amount: &str) -> bool {
    let _ = (to, amount);
    true
}

// vuln-code-snippet start testcodeCsrf027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let provided = req.header("X-CSRF-Token");
    let expected = req.cookie("csrf_session");
    if !constant_time_eq(provided.as_bytes(), expected.as_bytes()) {
        return super::shared::BenchmarkResponse::forbidden("csrf validation failed");
    }
    let to = req.param("to");
    let amount = req.param("amount");
    let result = transfer_funds(&to, &amount); // vuln-code-snippet target-line testcodeCsrf027
    if result {
        super::shared::BenchmarkResponse::ok("transfer complete")
    } else {
        super::shared::BenchmarkResponse::error("transfer failed")
    }
}
// vuln-code-snippet end testcodeCsrf027
