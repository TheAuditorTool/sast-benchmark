pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let result = cbor_deserialize(data.as_bytes());
    super::shared::BenchmarkResponse::ok(&result)
}

fn cbor_deserialize(_bytes: &[u8]) -> String {
    "cbor_object".to_string()
}
