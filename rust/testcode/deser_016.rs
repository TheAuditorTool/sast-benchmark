//! CWE-502: Typed struct deserialization with deny_unknown_fields rejecting extra data.

// vuln-code-snippet start testcodeDeser016
struct StrictRequest { name: String, age: u32 }

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let json_input = req.body_str();
    // Simulates: serde_json::from_str::<StrictRequest>() with #[serde(deny_unknown_fields)]
    let parsed = strict_deser(&json_input); // vuln-code-snippet target-line testcodeDeser016
    super::shared::BenchmarkResponse::ok(&format!("Name: {}", parsed))
}
fn strict_deser(input: &str) -> String {
    // Only accepts {"name": "...", "age": N}, rejects unknown fields
    format!("strict_parsed(len={})", input.len())
}
// vuln-code-snippet end testcodeDeser016
