//! CWE-798: Secret wrapped in secrecy::Secret type loaded from configuration file.

// vuln-code-snippet start testcodeHardcodedcreds019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");
    // Simulates: secrecy::Secret<String> loaded from config
    let secret = load_secret_from_config("api_key"); // vuln-code-snippet target-line testcodeHardcodedcreds019
    let result = format!("Action {} with secret [REDACTED]", action);
    super::shared::BenchmarkResponse::ok(&result)
}

fn load_secret_from_config(key: &str) -> String {
    // Simulates: config::get::<secrecy::SecretString>(key)
    format!("secret_from_config_{}", key)
}
// vuln-code-snippet end testcodeHardcodedcreds019
