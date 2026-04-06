//! CWE-502: YAML deserialization from user input allowing anchor-based amplification.

// vuln-code-snippet start testcodeDeser012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let yaml_input = req.body_str();
    // Simulates: serde_yaml::from_str(&yaml_input) -- YAML anchors can cause DoS
    let value = serde_yaml_from_str(&yaml_input); // vuln-code-snippet target-line testcodeDeser012
    super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", value))
}
fn serde_yaml_from_str(input: &str) -> String { format!("yaml_value(len={})", input.len()) }
// vuln-code-snippet end testcodeDeser012
