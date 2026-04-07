//! CWE-285: System configuration read without privilege check

fn access_system_config() -> String {
    "db_password=secret;api_key=abc123".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _key = req.param("key");
    let result = access_system_config(); // vuln-code-snippet target-line testcodeAuthzfailure010
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure010
