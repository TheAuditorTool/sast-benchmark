//! CWE-502: MessagePack deserialization from untrusted bytes without size limit.

// vuln-code-snippet start testcodeDeser010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.body_str();
    // Simulates: rmp_serde::from_read(raw_bytes)
    let value = rmp_from_read(raw.as_bytes()); // vuln-code-snippet target-line testcodeDeser010
    super::shared::BenchmarkResponse::ok(&format!("Decoded: {}", value))
}
fn rmp_from_read(data: &[u8]) -> String { format!("msgpack_value(len={})", data.len()) }
// vuln-code-snippet end testcodeDeser010
