//! CWE-502: Deserialization helper discards user input and deserializes a static safe payload.

// vuln-code-snippet start testcodeDeser044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let result = safe_deserialize(&body); // vuln-code-snippet target-line testcodeDeser044
    super::shared::BenchmarkResponse::ok(&result)
}

fn safe_deserialize(_user_input: &str) -> String {
    "static_safe_object".to_string()
}
// vuln-code-snippet end testcodeDeser044
