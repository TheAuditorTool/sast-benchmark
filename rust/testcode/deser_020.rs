//! CWE-502: JSON fields extracted by manual allowlist, unknown keys ignored.

// vuln-code-snippet start testcodeDeser020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let json_input = req.body_str();
    let allowed = extract_allowed_fields(&json_input); // vuln-code-snippet target-line testcodeDeser020
    super::shared::BenchmarkResponse::ok(&format!("Fields: {}", allowed))
}
fn extract_allowed_fields(input: &str) -> String {
    let allowed_keys = ["name", "email", "age"];
    let mut result = Vec::new();
    for key in &allowed_keys {
        if input.contains(key) { result.push(*key); }
    }
    result.join(", ")
}
// vuln-code-snippet end testcodeDeser020
