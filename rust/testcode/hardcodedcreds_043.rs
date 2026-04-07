//! CWE-798: Configuration loaded from a runtime-specified file path, not hardcoded.

// vuln-code-snippet start testcodeHardcodedcreds043
fn read_config_file(key_name: &str) -> String {
    std::env::var("CONFIG_PATH")
        .map(|p| format!("loaded '{}' from {}", key_name, p))
        .unwrap_or_default()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let service = req.param("service");
    let config_value = read_config_file(&service); // vuln-code-snippet target-line testcodeHardcodedcreds043
    let result = format!("Config for {}: {}", service, config_value);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds043
