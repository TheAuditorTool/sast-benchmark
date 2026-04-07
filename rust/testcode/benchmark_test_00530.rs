pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.body_str();
    let value = ciborium_from_reader(raw.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("Decoded: {}", value))
}
fn ciborium_from_reader(data: &[u8]) -> String { format!("cbor_value(len={})", data.len()) }
