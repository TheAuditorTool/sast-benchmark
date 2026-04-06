//! CWE-502: Protocol buffer message decoded with schema-defined types only.

// vuln-code-snippet start testcodeDeser021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.body_str();
    // Simulates: prost::Message::decode(raw_bytes) -- schema-defined types
    let msg = protobuf_decode(raw.as_bytes()); // vuln-code-snippet target-line testcodeDeser021
    super::shared::BenchmarkResponse::ok(&format!("Message: {}", msg))
}
fn protobuf_decode(data: &[u8]) -> String {
    // Simulates: prost::Message::decode() -- only schema-defined fields accepted
    format!("proto_msg(len={})", data.len())
}
// vuln-code-snippet end testcodeDeser021
