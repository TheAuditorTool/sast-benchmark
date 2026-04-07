//! CWE-352: CSRF token in URL query param for payment endpoint — token leaks via Referer/logs.

fn process_payment(amount: &str, card: &str) -> bool {
    let _ = (amount, card);
    true
}

fn get_expected_token() -> String {
    "expected_session_token".to_string()
}

// vuln-code-snippet start testcodeCsrf024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // CSRF token in URL — logged by load balancers, CDN, and application servers.
    let token = req.param("csrf_token");
    let expected = get_expected_token();
    if token == expected {
        let amount = req.param("amount");
        let card = req.param("card");
        let result = process_payment(&amount, &card); // vuln-code-snippet target-line testcodeCsrf024
        if result {
            return super::shared::BenchmarkResponse::ok("payment processed");
        }
        return super::shared::BenchmarkResponse::error("payment failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
// vuln-code-snippet end testcodeCsrf024
