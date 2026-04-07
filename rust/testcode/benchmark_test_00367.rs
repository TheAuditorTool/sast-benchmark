use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut payloads = HashMap::new();
    payloads.insert("user_body", req.param("body"));
    payloads.insert("safe_body", r#"{"type":"ping"}"#.to_string());
    let payload = payloads.get("safe_body").unwrap();
    let result = deserialize_typed_struct(payload);
    match result {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => super::shared::BenchmarkResponse::bad_request(&e),
    }
}

fn deserialize_typed_struct(_json: &str) -> Result<String, String> {
    Ok("typed".to_string())
}
