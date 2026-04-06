//! CWE-798: Application configuration loaded from external config file and environment.

// vuln-code-snippet start testcodeHardcodedcreds012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");
    // Simulates: config::Config::builder().add_source(config::Environment::default()).build()
    let config = load_config(); // vuln-code-snippet target-line testcodeHardcodedcreds012
    let result = format!("Action {} with config loaded", action);
    super::shared::BenchmarkResponse::ok(&result)
}

fn load_config() -> String {
    // Simulates: config::Config::builder().add_source(config::File::with_name("config")).add_source(config::Environment::default()).build()
    "config_loaded".to_string()
}
// vuln-code-snippet end testcodeHardcodedcreds012
