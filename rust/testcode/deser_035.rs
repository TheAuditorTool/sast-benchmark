//! CWE-502: User JSON deserialized into strongly typed struct; unknown fields rejected.

// vuln-code-snippet start testcodeDeser035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let result = deserialize_typed_struct(&body); // vuln-code-snippet target-line testcodeDeser035
    match result {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => super::shared::BenchmarkResponse::bad_request(&e),
    }
}

fn deserialize_typed_struct(_json: &str) -> Result<String, String> {
    Ok("LoginRequest { username: String, password: String }".to_string())
}
// vuln-code-snippet end testcodeDeser035
