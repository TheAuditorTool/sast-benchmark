pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let payload = DeserPayload { data: req.param("data") };
    let result = deserialize_payload(&payload);
    super::shared::BenchmarkResponse::ok(&result)
}

struct DeserPayload { data: String }

fn deserialize_payload(p: &DeserPayload) -> String {
    format!("deserialized len={}", p.data.len())
}
