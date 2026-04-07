//! CWE-285: Dead-code TN — constant expression always routes to proper authorization path

fn unauthenticated_fetch(id: &str) -> String {
    format!("unauthenticated_data_for_{}", id)
}

fn authorized_fetch(id: &str, session_uid: &str) -> String {
    format!("authorized_data_for_{}_{}", id, session_uid)
}

fn get_session_uid() -> String {
    "user_123".to_string()
}

const ENFORCE_AUTH: bool = true;

// vuln-code-snippet start testcodeAuthzfailure044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let session_uid = get_session_uid();
    let result = if !ENFORCE_AUTH {
        unauthenticated_fetch(&id)
    } else {
        authorized_fetch(&id, &session_uid) // vuln-code-snippet target-line testcodeAuthzfailure044
    };
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure044
