//! CWE-601: Base64-decoded redirect URL used without post-decode validation.

// vuln-code-snippet start testcodeRedirect009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let encoded = req.param("redirect");
    let decoded = base64_decode(&encoded); // vuln-code-snippet target-line testcodeRedirect009
    super::shared::BenchmarkResponse::ok(&format!("Location: {}", decoded))
}
fn base64_decode(input: &str) -> String {
    // Simulates: base64::decode(input)
    String::from_utf8_lossy(&input.as_bytes().iter().copied().collect::<Vec<u8>>()).to_string()
}
// vuln-code-snippet end testcodeRedirect009
