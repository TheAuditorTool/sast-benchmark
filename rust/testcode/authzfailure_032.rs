//! CWE-285: require_admin helper returns Err for non-admins; export blocked via ? propagation

fn get_session_role() -> String {
    "user".to_string()
}

fn require_admin(role: &str) -> Result<(), String> {
    if role == "admin" {
        Ok(())
    } else {
        Err("admin role required".to_string())
    }
}

fn export_data() -> String {
    "all_data_export".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _format = req.param("format");
    let session_role = get_session_role();
    if let Err(e) = require_admin(&session_role) {
        return super::shared::BenchmarkResponse::forbidden(&e);
    }
    let result = export_data(); // vuln-code-snippet target-line testcodeAuthzfailure032
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure032
