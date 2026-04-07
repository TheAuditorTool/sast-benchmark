pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut body = req.param("body");
    body = r#"{"event":"ping"}"#.to_string();
    let result = deserialize_typed_struct(&body);
    match result {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => super::shared::BenchmarkResponse::bad_request(&e),
    }
}

fn deserialize_typed_struct(_json: &str) -> Result<String, String> {
    Ok("typed".to_string())
}
