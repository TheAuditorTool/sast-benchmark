//! CWE-798: Redis connection URL with embedded password in const.

// vuln-code-snippet start testcodeHardcodedcreds010
const REDIS_URL: &str = "redis://:p4ssw0rd@redis:6379/0"; // vuln-code-snippet target-line testcodeHardcodedcreds010

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let cache_key = req.param("key");
    // Simulates: redis::Client::open(REDIS_URL)
    let result = format!("Cache lookup for key '{}' at {}", cache_key, REDIS_URL);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds010
