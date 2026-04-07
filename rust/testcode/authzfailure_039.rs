//! CWE-285: Role extracted from server-side session store, not from request; action gated

fn get_session_role(session_token: &str) -> String {
    // Simulates lookup in server-side session store by token
    let _ = session_token;
    "user".to_string()
}

fn privileged_action() -> String {
    "sensitive_operation_result".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let session_token = req.cookie("session");
    let role = get_session_role(&session_token);
    if role != "admin" {
        return super::shared::BenchmarkResponse::forbidden("insufficient privileges");
    }
    let result = privileged_action(); // vuln-code-snippet target-line testcodeAuthzfailure039
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure039
