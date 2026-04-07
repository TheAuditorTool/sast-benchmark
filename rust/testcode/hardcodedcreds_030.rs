//! CWE-798: Full RSA private key stored as a hardcoded const string.

// vuln-code-snippet start testcodeHardcodedcreds030
const SSH_PRIVATE_KEY: &str = "-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEA...\n-----END RSA PRIVATE KEY-----"; // vuln-code-snippet target-line testcodeHardcodedcreds030

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let host = req.param("host");
    let result = format!("Connecting to {} using embedded RSA key ({} bytes)", host, SSH_PRIVATE_KEY.len());
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds030
