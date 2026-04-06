//! CWE-502: Tagged enum deserialization requiring known discriminant value.

// vuln-code-snippet start testcodeDeser018
enum RequestType { Create, Update, Delete }

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let json_input = req.body_str();
    // Simulates: #[serde(tag = "type")] enum -- only known variants accepted
    let request_type = tagged_enum_deser(&json_input); // vuln-code-snippet target-line testcodeDeser018
    super::shared::BenchmarkResponse::ok(&format!("Type: {}", request_type))
}
fn tagged_enum_deser(input: &str) -> String {
    // Only "Create", "Update", "Delete" accepted as discriminant
    if input.contains("Create") { "Create".into() }
    else if input.contains("Update") { "Update".into() }
    else if input.contains("Delete") { "Delete".into() }
    else { "rejected_unknown_variant".into() }
}
// vuln-code-snippet end testcodeDeser018
