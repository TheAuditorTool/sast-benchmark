pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let result = bincode_deserialize(data.as_bytes());
    super::shared::BenchmarkResponse::ok(&result)
}

fn bincode_deserialize(_bytes: &[u8]) -> String {
    "deserialized_object".to_string()
}
