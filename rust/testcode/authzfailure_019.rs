//! CWE-285: Role check occurs after the sensitive action has already been performed

fn perform_action(data: &str) -> String {
    format!("action_performed_with_{}", data)
}

fn is_admin() -> bool {
    false
}

fn log_unauthorized() {
    let _ = "unauthorized_access_logged";
}

// vuln-code-snippet start testcodeAuthzfailure019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let result = perform_action(&data); // vuln-code-snippet target-line testcodeAuthzfailure019
    if !is_admin() {
        log_unauthorized();
    }
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure019
