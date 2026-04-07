//! CWE-285: Dead-code TN — `skip` is always false; authorized_action always taken

fn unauthorized_action(id: &str) -> String {
    format!("unauthorized_data_for_{}", id)
}

fn authorized_action(id: &str, session_uid: &str) -> String {
    format!("authorized_data_for_{}_{}", id, session_uid)
}

fn get_session_uid() -> String {
    "user_123".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure045
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let session_uid = get_session_uid();
    let skip = 0 > 1;
    let result = if skip {
        unauthorized_action(&id)
    } else {
        authorized_action(&id, &session_uid) // vuln-code-snippet target-line testcodeAuthzfailure045
    };
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure045
