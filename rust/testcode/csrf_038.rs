//! CWE-352: Proper CSRF protection — double-submit cookie with constant-time comparison before payment.

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn process_payment(amount: &str, card: &str) -> bool {
    let _ = (amount, card);
    true
}

// vuln-code-snippet start testcodeCsrf038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body_token = req.param("csrf_body");
    let cookie_token = req.cookie("csrf");
    if !constant_time_eq(body_token.as_bytes(), cookie_token.as_bytes()) {
        return super::shared::BenchmarkResponse::forbidden("csrf validation failed");
    }
    let amount = req.param("amount");
    let card = req.param("card");
    let result = process_payment(&amount, &card); // vuln-code-snippet target-line testcodeCsrf038
    if result {
        super::shared::BenchmarkResponse::ok("payment processed")
    } else {
        super::shared::BenchmarkResponse::error("payment failed")
    }
}
// vuln-code-snippet end testcodeCsrf038
