//! CWE-502: CBOR deserialization from request body without schema validation.

// vuln-code-snippet start testcodeDeser011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.body_str();
    // Simulates: ciborium::from_reader(body)
    let value = ciborium_from_reader(raw.as_bytes()); // vuln-code-snippet target-line testcodeDeser011
    super::shared::BenchmarkResponse::ok(&format!("Decoded: {}", value))
}
fn ciborium_from_reader(data: &[u8]) -> String { format!("cbor_value(len={})", data.len()) }
// vuln-code-snippet end testcodeDeser011
