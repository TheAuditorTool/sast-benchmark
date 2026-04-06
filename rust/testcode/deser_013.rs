//! CWE-502: No-std binary format deserialized from untrusted embedded device data.

// vuln-code-snippet start testcodeDeser013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.body_str();
    // Simulates: postcard::from_bytes(raw_bytes)
    let value = postcard_from_bytes(raw.as_bytes()); // vuln-code-snippet target-line testcodeDeser013
    super::shared::BenchmarkResponse::ok(&format!("Device data: {}", value))
}
fn postcard_from_bytes(data: &[u8]) -> String { format!("postcard_value(len={})", data.len()) }
// vuln-code-snippet end testcodeDeser013
