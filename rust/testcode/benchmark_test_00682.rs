pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let obj = arbitrary_deserialize(data.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("type={}", obj))
}

fn arbitrary_deserialize(_bytes: &[u8]) -> String {
    "ArbitraryObject".to_string()
}
