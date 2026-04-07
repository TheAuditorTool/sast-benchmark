pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    if body.len() > 65_536 {
        return super::shared::BenchmarkResponse::bad_request("Payload too large");
    }
    let result = deserialize_typed_struct(&body);
    match result {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => super::shared::BenchmarkResponse::bad_request(&e),
    }
}

fn deserialize_typed_struct(_json: &str) -> Result<String, String> {
    Ok("struct parsed".to_string())
}
