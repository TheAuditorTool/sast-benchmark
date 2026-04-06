//! CWE-502: Custom deserializer that accepts arbitrary structure without field validation.

// vuln-code-snippet start testcodeDeser015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let json_input = req.body_str();
    // Custom deserializer with no field validation
    let value = permissive_deser(&json_input); // vuln-code-snippet target-line testcodeDeser015
    super::shared::BenchmarkResponse::ok(&format!("Data: {}", value))
}
fn permissive_deser(input: &str) -> String {
    // Simulates: serde_json::from_str::<serde_json::Value>() -- accepts any shape
    format!("any_value(len={})", input.len())
}
// vuln-code-snippet end testcodeDeser015
