//! CWE-798: Hardcoded Redis AUTH password stored in a const.

// vuln-code-snippet start testcodeHardcodedcreds027
const REDIS_PASSWORD: &str = "redis-master-password-prod"; // vuln-code-snippet target-line testcodeHardcodedcreds027

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let key = req.param("key");
    let result = format!(
        "AUTH {} | GET {}",
        REDIS_PASSWORD, key
    );
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds027
