//! CWE-798: PEM private key embedded as a const string in source.

// vuln-code-snippet start testcodeHardcodedcreds005
const PRIVATE_KEY_PEM: &str = "-----BEGIN RSA PRIVATE KEY-----\nMIIEpAIBAAKCAQEA0Z3VS5JJcds3xfn/ygWep4PAtGoRBh...truncated\n-----END RSA PRIVATE KEY-----"; // vuln-code-snippet target-line testcodeHardcodedcreds005

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let message = req.param("message");
    // Simulates: rsa::RsaPrivateKey::from_pkcs1_pem(PRIVATE_KEY_PEM)
    let signed = format!("signed({})", message);
    super::shared::BenchmarkResponse::ok(&format!("Signature: {}", signed))
}
// vuln-code-snippet end testcodeHardcodedcreds005
