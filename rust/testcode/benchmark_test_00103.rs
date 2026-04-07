pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.body_str();
    let msg = protobuf_decode(raw.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("Message: {}", msg))
}
fn protobuf_decode(data: &[u8]) -> String {
    format!("proto_msg(len={})", data.len())
}
