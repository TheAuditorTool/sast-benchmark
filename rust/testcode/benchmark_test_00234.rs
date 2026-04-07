const MAX_BODY_SIZE: usize = 1_048_576;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.body_str();

    if body.len() > MAX_BODY_SIZE {
        return super::shared::BenchmarkResponse::bad_request("Payload too large");
    }

    let parsed: serde_json::Value = serde_json::from_str(&body)
        .unwrap_or(serde_json::Value::Null);

    super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", parsed))
}
