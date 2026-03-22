//! CWE-502: YAML input parsed without size limit.

// vuln-code-snippet start testcodeDeser002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let yaml_input = req.body_str();

    // No size limit — YAML bomb (billion laughs via alias expansion) can exhaust memory
    let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml_input) // vuln-code-snippet target-line testcodeDeser002
        .unwrap_or(serde_yaml::Value::Null);

    super::shared::BenchmarkResponse::ok(&format!("Parsed YAML: {:?}", parsed))
}
// vuln-code-snippet end testcodeDeser002
