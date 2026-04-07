pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.body_str();
    let value = rmp_from_read(raw.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("Decoded: {}", value))
}
fn rmp_from_read(data: &[u8]) -> String { format!("msgpack_value(len={})", data.len()) }
