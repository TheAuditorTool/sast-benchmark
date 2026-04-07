//! CWE-285: Authorization logic always evaluates true due to short-circuit `|| true`

struct Resource {
    owner_id: String,
    content: String,
}

fn db_get_resource(id: &str) -> Resource {
    Resource {
        owner_id: "user_456".to_string(),
        content: format!("resource_content_for_{}", id),
    }
}

fn get_session_user_id() -> String {
    "user_123".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let resource = db_get_resource(&id);
    let session = get_session_user_id();
    if resource.owner_id == session || true {
        return super::shared::BenchmarkResponse::ok(&resource.content); // vuln-code-snippet target-line testcodeAuthzfailure018
    }
    super::shared::BenchmarkResponse::forbidden("access denied")
}
// vuln-code-snippet end testcodeAuthzfailure018
