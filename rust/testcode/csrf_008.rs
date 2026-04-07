//! CWE-352: CSRF token only logged, not validated, before action is performed.

fn tracing_log(msg: &str) {
    let _ = msg;
}

fn perform_action(action: &str) -> bool {
    let _ = action;
    true
}

// vuln-code-snippet start testcodeCsrf008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    tracing_log(&format!("csrf={}", req.param("csrf_token"))); // logged but not validated
    let action = req.param("action");
    let result = perform_action(&action); // vuln-code-snippet target-line testcodeCsrf008
    if result {
        super::shared::BenchmarkResponse::ok("action performed")
    } else {
        super::shared::BenchmarkResponse::error("action failed")
    }
}
// vuln-code-snippet end testcodeCsrf008
