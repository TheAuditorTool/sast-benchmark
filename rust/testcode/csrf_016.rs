//! CWE-352: Non-constant-time string equality for CSRF token before password change (timing oracle).

fn change_password(pwd: &str) -> bool {
    let _ = pwd;
    true
}

// vuln-code-snippet start testcodeCsrf016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // == short-circuits on the first differing byte, leaking timing information.
    if req.param("csrf") == req.cookie("csrf_session") {
        let pwd = req.param("pwd");
        let result = change_password(&pwd); // vuln-code-snippet target-line testcodeCsrf016
        if result {
            return super::shared::BenchmarkResponse::ok("password changed");
        }
        return super::shared::BenchmarkResponse::error("change failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
// vuln-code-snippet end testcodeCsrf016
