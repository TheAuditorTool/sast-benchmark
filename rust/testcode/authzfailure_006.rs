//! CWE-285: Admin operation performed without any role check

fn delete_all_users() -> String {
    "all_users_deleted".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let result = delete_all_users(); // vuln-code-snippet target-line testcodeAuthzfailure006
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure006
