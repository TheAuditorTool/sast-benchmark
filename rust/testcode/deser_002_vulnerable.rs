//! Insecure Deserialization True Positive — CWE-502
//! YAML input parsed without size limit. A YAML bomb (billion laughs)
//! can exhaust memory and cause denial of service.

// vuln-code-snippet start testcodeDeser002Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let yaml_input = req.body_str();

    // VULNERABLE: No size limit — YAML bomb can exhaust memory
    let parsed: serde_json::Value = yaml_parse(&yaml_input); // vuln-code-snippet vuln-line testcodeDeser002Vulnerable

    super::shared::BenchmarkResponse::ok(&format!("Parsed YAML: {}", parsed))
}

fn yaml_parse(input: &str) -> serde_json::Value {
    // Simulates serde_yaml::from_str — no size or depth limit
    serde_json::from_str(input).unwrap_or(serde_json::Value::Null)
}
// vuln-code-snippet end testcodeDeser002Vulnerable
