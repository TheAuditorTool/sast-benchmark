//! CWE-285: User-controlled superadmin query parameter trusted for panel access

fn superadmin_panel() -> String {
    "superadmin_panel_data".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.param("superadmin") == "1" {
        let result = superadmin_panel(); // vuln-code-snippet target-line testcodeAuthzfailure015
        return super::shared::BenchmarkResponse::ok(&result);
    }
    super::shared::BenchmarkResponse::forbidden("access denied")
}
// vuln-code-snippet end testcodeAuthzfailure015
