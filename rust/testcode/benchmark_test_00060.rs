pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let obj = deserialize_object(body.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("deserialized len={}", obj))
}

fn deserialize_object(_data: &[u8]) -> usize { 42 }
