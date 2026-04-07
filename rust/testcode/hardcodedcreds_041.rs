//! CWE-798: Unused placeholder variable shadowed by actual env-sourced secret.

// vuln-code-snippet start testcodeHardcodedcreds041
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let resource = req.param("resource");
    let secret = "placeholder-not-real"; // vuln-code-snippet target-line testcodeHardcodedcreds041
    let actual_secret = std::env::var("SECRET").unwrap_or_default();
    let _ = secret; // placeholder is intentionally unused
    let result = format!("Accessing {} with secret_len={}", resource, actual_secret.len());
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds041
