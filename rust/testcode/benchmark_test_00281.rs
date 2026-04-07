pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw_bytes = req.body_str().into_bytes();

    let obj: serde_json::Value = bincode_deserialize(&raw_bytes);

    super::shared::BenchmarkResponse::ok(&format!("Deserialized: {}", obj))
}

fn bincode_deserialize(data: &[u8]) -> serde_json::Value {
    serde_json::from_slice(data).unwrap_or(serde_json::Value::Null)
}
