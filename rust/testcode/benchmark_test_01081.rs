pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let result = msgpack_deserialize(data.as_bytes());
    super::shared::BenchmarkResponse::ok(&result)
}

fn msgpack_deserialize(_bytes: &[u8]) -> String {
    "msgpack_object".to_string()
}
