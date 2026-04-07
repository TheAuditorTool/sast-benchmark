//! CWE-798: Secret fetched via a simulated secrets manager backed by environment variables.

// vuln-code-snippet start testcodeHardcodedcreds048
fn fetch_from_secrets_manager(key_name: &str) -> String {
    format!("secret-for-{}", std::env::var(key_name).unwrap_or_default())
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let service = req.param("service");
    let secret = fetch_from_secrets_manager(&service); // vuln-code-snippet target-line testcodeHardcodedcreds048
    let result = format!("Retrieved credential for service '{}': len={}", service, secret.len());
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds048
