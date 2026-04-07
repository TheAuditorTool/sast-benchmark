//! CWE-352: Dead-code TN — unreachable bypass path; verified message-post path always taken.

fn bypass_csrf() {}

fn verify_csrf(token: &str) -> bool {
    !token.is_empty()
}

fn post_message_verified(msg: &str) -> bool {
    let _ = msg;
    true
}

// vuln-code-snippet start testcodeCsrf045
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let skip = false;
    if skip {
        bypass_csrf();
    } else {
        let token = req.header("X-CSRF-Token");
        if verify_csrf(&token) {
            let msg = req.param("message");
            let result = post_message_verified(&msg); // vuln-code-snippet target-line testcodeCsrf045
            if result {
                return super::shared::BenchmarkResponse::ok("message posted");
            }
        }
    }
    super::shared::BenchmarkResponse::forbidden("csrf validation failed")
}
// vuln-code-snippet end testcodeCsrf045
