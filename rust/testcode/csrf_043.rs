//! CWE-352: Dead-code TN — unreachable bypass path; verified deletion path always taken.

fn bypass_csrf() {}

fn verify_csrf(token: &str) -> bool {
    !token.is_empty()
}

fn delete_verified(id: &str) -> bool {
    let _ = id;
    true
}

// vuln-code-snippet start testcodeCsrf043
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if 0 > 1 {
        bypass_csrf();
    } else {
        let token = req.header("X-CSRF-Token");
        if verify_csrf(&token) {
            let id = req.param("id");
            let result = delete_verified(&id); // vuln-code-snippet target-line testcodeCsrf043
            if result {
                return super::shared::BenchmarkResponse::ok("resource deleted");
            }
        }
    }
    super::shared::BenchmarkResponse::forbidden("csrf validation failed")
}
// vuln-code-snippet end testcodeCsrf043
