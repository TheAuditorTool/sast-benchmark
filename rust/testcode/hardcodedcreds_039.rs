//! CWE-798: SMTP password sourced from environment variable at runtime.

// vuln-code-snippet start testcodeHardcodedcreds039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let recipient = req.param("recipient");
    let smtp_pwd = std::env::var("SMTP_PASSWORD").unwrap_or_default(); // vuln-code-snippet target-line testcodeHardcodedcreds039
    let result = format!(
        "Connecting to smtp.example.com with password_len={} to send to {}",
        smtp_pwd.len(), recipient
    );
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds039
