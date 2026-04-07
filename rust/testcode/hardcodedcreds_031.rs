//! CWE-798: Hardcoded AES-256 encryption key as a byte string literal.

// vuln-code-snippet start testcodeHardcodedcreds031
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = b"hardcoded-aes-256-key-32bytes!!!"; // vuln-code-snippet target-line testcodeHardcodedcreds031
    let result = format!("Encrypting '{}' with key of {} bytes", plaintext, key.len());
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds031
