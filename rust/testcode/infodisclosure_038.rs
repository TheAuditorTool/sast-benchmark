//! CWE-200: Secret loaded from environment but used internally; never included in response.

// vuln-code-snippet start testcodeInfodisclosure038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _id = req.param("id");
    let _secret = std::env::var("SECRET_KEY").unwrap_or_default();
    let result = verify_with_secret(&_secret);
    super::shared::BenchmarkResponse::ok(&format!("verified={}", result)) // vuln-code-snippet target-line testcodeInfodisclosure038
}

fn verify_with_secret(_secret: &str) -> bool { true }
// vuln-code-snippet end testcodeInfodisclosure038
