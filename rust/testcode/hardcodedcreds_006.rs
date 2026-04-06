//! CWE-798: SMTP configuration struct initialized with password literal.

// vuln-code-snippet start testcodeHardcodedcreds006
struct SmtpConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let recipient = req.param("to");
    let config = SmtpConfig {
        host: "smtp.example.com".to_string(),
        port: 587,
        username: "noreply@example.com".to_string(),
        password: "mailpass123".to_string(), // vuln-code-snippet target-line testcodeHardcodedcreds006
    };
    let result = format!("Email sent to {} via {}:{}", recipient, config.host, config.port);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds006
