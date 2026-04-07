//! CWE-798: Config struct initialized with a hardcoded production API key literal.

// vuln-code-snippet start testcodeHardcodedcreds025
struct AppConfig {
    api_key: &'static str,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let cfg = AppConfig { api_key: "hardcoded-prod-key-xyz789abc" }; // vuln-code-snippet target-line testcodeHardcodedcreds025
    let result = format!("Calling {} with key={}", endpoint, cfg.api_key);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds025
