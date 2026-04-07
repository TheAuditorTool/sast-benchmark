//! CWE-285: User-controlled role header trusted for authorization decision

fn admin_action() -> String {
    "admin_action_performed".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.header("X-Role") == "admin" {
        let result = admin_action(); // vuln-code-snippet target-line testcodeAuthzfailure011
        return super::shared::BenchmarkResponse::ok(&result);
    }
    super::shared::BenchmarkResponse::forbidden("access denied")
}
// vuln-code-snippet end testcodeAuthzfailure011
