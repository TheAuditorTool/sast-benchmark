//! CWE-798: Secret sourced from environment variable via Option, no hardcoded fallback.

// vuln-code-snippet start testcodeHardcodedcreds047
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let resource = req.param("resource");
    let secret = std::env::var("SECRET").ok(); // vuln-code-snippet target-line testcodeHardcodedcreds047
    match secret {
        Some(s) => {
            let result = format!("Accessing {} with secret_len={}", resource, s.len());
            super::shared::BenchmarkResponse::ok(&result)
        }
        None => super::shared::BenchmarkResponse::error("SECRET not set"),
    }
}
// vuln-code-snippet end testcodeHardcodedcreds047
