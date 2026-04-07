//! CWE-285: Dead-code TN — constant-false bypass always routes to authorized path

fn bypass_auth_and_fetch(id: &str) -> String {
    format!("unrestricted_data_for_{}", id)
}

fn fetch_with_auth_check(id: &str, session_uid: &str) -> String {
    format!("authorized_data_for_{}_{}", id, session_uid)
}

fn get_session_uid() -> String {
    "user_123".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure042
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let session_uid = get_session_uid();
    let bypass = false;
    let result = if bypass {
        bypass_auth_and_fetch(&id)
    } else {
        fetch_with_auth_check(&id, &session_uid) // vuln-code-snippet target-line testcodeAuthzfailure042
    };
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure042
