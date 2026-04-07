//! CWE-798: Hardcoded SMTP password stored in a const used in connection setup.

// vuln-code-snippet start testcodeHardcodedcreds026
const SMTP_PASSWORD: &str = "Smtp@P4ssw0rd2024"; // vuln-code-snippet target-line testcodeHardcodedcreds026

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let recipient = req.param("recipient");
    let result = format!(
        "Connecting to smtp.example.com with password={} to send mail to {}",
        SMTP_PASSWORD, recipient
    );
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds026
