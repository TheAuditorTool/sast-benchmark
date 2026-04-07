//! CWE-285: User-controlled permission level header trusted for privileged operation

fn privileged_op() -> String {
    "privileged_operation_executed".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let level = req.header("X-Permission-Level").parse::<u32>().unwrap_or(0);
    if level >= 10 {
        let result = privileged_op(); // vuln-code-snippet target-line testcodeAuthzfailure014
        return super::shared::BenchmarkResponse::ok(&result);
    }
    super::shared::BenchmarkResponse::forbidden("insufficient permission level")
}
// vuln-code-snippet end testcodeAuthzfailure014
