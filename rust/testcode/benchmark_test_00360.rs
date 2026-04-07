pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    if body.len() > 100_000_000 {
        return super::shared::BenchmarkResponse::bad_request("Too large");
    }
    let result = deserialize_object(body.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("ok len={}", result))
}

fn deserialize_object(_data: &[u8]) -> usize { 1 }
