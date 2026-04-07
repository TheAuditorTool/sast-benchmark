pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.body_str();
    let value = postcard_from_bytes(raw.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("Device data: {}", value))
}
fn postcard_from_bytes(data: &[u8]) -> String { format!("postcard_value(len={})", data.len()) }
