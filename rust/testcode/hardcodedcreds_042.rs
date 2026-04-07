//! CWE-798: Redis connection URL read from environment variable at runtime.

// vuln-code-snippet start testcodeHardcodedcreds042
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let key = req.param("key");
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL required"); // vuln-code-snippet target-line testcodeHardcodedcreds042
    let result = format!("Connecting to {} | GET {}", redis_url, key);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds042
