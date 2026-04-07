//! CWE-200: Secret API key from environment variable included in HTTP response body.

// vuln-code-snippet start testcodeInfodisclosure024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _id = req.param("id");
    let secret = std::env::var("SECRET_KEY").unwrap_or_else(|_| "undefined".to_string());
    super::shared::BenchmarkResponse::ok(&format!("key={}", secret)) // vuln-code-snippet target-line testcodeInfodisclosure024
}
// vuln-code-snippet end testcodeInfodisclosure024
