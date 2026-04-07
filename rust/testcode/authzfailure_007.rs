//! CWE-285: Admin data export performed without authorization check

fn export_all_data() -> String {
    "all_user_data_export".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _format = req.param("format");
    let result = export_all_data(); // vuln-code-snippet target-line testcodeAuthzfailure007
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure007
