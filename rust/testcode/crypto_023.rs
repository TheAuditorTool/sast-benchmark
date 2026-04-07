//! CWE-327: XOR with fixed key used as encryption; trivially reversible without key secrecy.

// vuln-code-snippet start testcodeCrypto023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = 0x42u8;
    let encrypted: Vec<u8> = plaintext.bytes().map(|b| b ^ key).collect(); // vuln-code-snippet target-line testcodeCrypto023
    super::shared::BenchmarkResponse::ok(&format!("encrypted_len={}", encrypted.len()))
}
// vuln-code-snippet end testcodeCrypto023
