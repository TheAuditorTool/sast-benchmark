//! CWE-352: Structural TN — CSRF bypass variable overwritten to false before check; safe path always taken.

fn skip_csrf() {}

fn verify_csrf_token(token: &str) -> bool {
    !token.is_empty()
}

fn change_data(data: &str) -> bool {
    let _ = data;
    true
}

// vuln-code-snippet start testcodeCsrf049
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut do_skip = true;
    do_skip = false; // overwritten — bypass is never reachable
    if do_skip {
        skip_csrf();
    } else {
        let token = req.header("X-CSRF-Token");
        if verify_csrf_token(&token) {
            let data = req.param("data");
            let result = change_data(&data); // vuln-code-snippet target-line testcodeCsrf049
            if result {
                return super::shared::BenchmarkResponse::ok("data changed");
            }
        }
    }
    super::shared::BenchmarkResponse::forbidden("csrf validation failed")
}
// vuln-code-snippet end testcodeCsrf049
