//! CWE-502: User-uploaded TOML configuration file parsed without validation.

// vuln-code-snippet start testcodeDeser014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let toml_input = req.body_str();
    // Simulates: toml::from_str(&toml_input)
    let config = toml_from_str(&toml_input); // vuln-code-snippet target-line testcodeDeser014
    super::shared::BenchmarkResponse::ok(&format!("Config: {}", config))
}
fn toml_from_str(input: &str) -> String { format!("toml_value(len={})", input.len()) }
// vuln-code-snippet end testcodeDeser014
