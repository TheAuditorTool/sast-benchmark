//! CWE-352: Dead-code TN — constant-false bypass path; verified transfer path always taken.

fn bypass_csrf() {}

fn verify_csrf(token: &str) -> bool {
    !token.is_empty()
}

fn transfer_funds_verified(to: &str, amount: &str) -> bool {
    let _ = (to, amount);
    true
}

// vuln-code-snippet start testcodeCsrf042
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if false {
        bypass_csrf();
    } else {
        let token = req.header("X-CSRF-Token");
        if verify_csrf(&token) {
            let to = req.param("to");
            let amount = req.param("amount");
            let result = transfer_funds_verified(&to, &amount); // vuln-code-snippet target-line testcodeCsrf042
            if result {
                return super::shared::BenchmarkResponse::ok("transfer complete");
            }
        }
    }
    super::shared::BenchmarkResponse::forbidden("csrf validation failed")
}
// vuln-code-snippet end testcodeCsrf042
