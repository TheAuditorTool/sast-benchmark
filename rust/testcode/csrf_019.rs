//! CWE-352: Double-submit cookie pattern with non-constant-time == comparison (timing oracle).

fn post_message(msg: &str) -> bool {
    let _ = msg;
    true
}

// vuln-code-snippet start testcodeCsrf019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // Double-submit cookie intent, but == leaks timing information.
    if req.cookie("csrf") == req.header("X-CSRF-Token") {
        let msg = req.param("message");
        let result = post_message(&msg); // vuln-code-snippet target-line testcodeCsrf019
        if result {
            return super::shared::BenchmarkResponse::ok("message posted");
        }
        return super::shared::BenchmarkResponse::error("post failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
// vuln-code-snippet end testcodeCsrf019
