//! CWE-352: Structural TN — custom X-Requested-With header blocks CSRF via CORS preflight.

fn perform_action(action: &str) -> bool {
    let _ = action;
    true
}

// vuln-code-snippet start testcodeCsrf048
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // AJAX requests with custom headers require CORS preflight, which browsers block for cross-origin requests.
    if req.header("X-Requested-With") != "XMLHttpRequest" {
        return super::shared::BenchmarkResponse::forbidden("custom header required");
    }
    let action = req.param("action");
    let result = perform_action(&action); // vuln-code-snippet target-line testcodeCsrf048
    if result {
        super::shared::BenchmarkResponse::ok("action performed")
    } else {
        super::shared::BenchmarkResponse::error("action failed")
    }
}
// vuln-code-snippet end testcodeCsrf048
