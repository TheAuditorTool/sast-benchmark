//! CWE-285: User-controlled role query parameter trusted for superuser authorization

fn superuser_op() -> String {
    "superuser_operation_executed".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let role = req.param("role");
    if role == "superuser" {
        let result = superuser_op(); // vuln-code-snippet target-line testcodeAuthzfailure012
        return super::shared::BenchmarkResponse::ok(&result);
    }
    super::shared::BenchmarkResponse::forbidden("access denied")
}
// vuln-code-snippet end testcodeAuthzfailure012
