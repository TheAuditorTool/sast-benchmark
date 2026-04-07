//! CWE-798: Dead-code branch — hardcoded string is unreachable; real secret comes from env.

// vuln-code-snippet start testcodeHardcodedcreds037
fn get_secret() -> String {
    // Always reads from env; hardcoded value is dead code
    if 1 > 2 {
        "dead-hardcoded-value".to_string() // vuln-code-snippet target-line testcodeHardcodedcreds037
    } else {
        std::env::var("SECRET_KEY").unwrap_or_default()
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let resource = req.param("resource");
    let secret = get_secret();
    let result = format!("Accessing {} with secret length={}", resource, secret.len());
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds037
