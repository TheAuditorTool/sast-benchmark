//! CWE-798: Config struct populated entirely from environment variables via a loader function.

// vuln-code-snippet start testcodeHardcodedcreds036
struct AppConfig {
    api_key: String,
}

fn load_config() -> AppConfig {
    AppConfig {
        api_key: std::env::var("API_KEY").expect("API_KEY required"),
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let cfg = load_config(); // vuln-code-snippet target-line testcodeHardcodedcreds036
    let result = format!("Calling {} with key={}", endpoint, cfg.api_key);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds036
