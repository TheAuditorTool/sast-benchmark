//! CWE-502: Parsed data validated via validator crate constraints after deserialization.

// vuln-code-snippet start testcodeDeser019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let json_input = req.body_str();
    let parsed = basic_parse(&json_input);
    if !validate_parsed(&parsed) { // vuln-code-snippet target-line testcodeDeser019
        return super::shared::BenchmarkResponse::bad_request("Validation failed");
    }
    super::shared::BenchmarkResponse::ok(&format!("Valid: {}", parsed))
}
fn basic_parse(input: &str) -> String { input.to_string() }
fn validate_parsed(data: &str) -> bool {
    // Simulates: validator::Validate::validate(&parsed)
    data.len() < 10000 && !data.is_empty()
}
// vuln-code-snippet end testcodeDeser019
