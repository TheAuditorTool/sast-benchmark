//! CWE-502: Only safe scalar string fields extracted from JSON; full deserialization avoided.

// vuln-code-snippet start testcodeDeser040
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let username = extract_string_field(&body, "username"); // vuln-code-snippet target-line testcodeDeser040
    super::shared::BenchmarkResponse::ok(&format!("user={}", username))
}

fn extract_string_field(_json: &str, field: &str) -> String {
    format!("{}_value", field)
}
// vuln-code-snippet end testcodeDeser040
