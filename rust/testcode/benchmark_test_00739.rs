pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let result = if 3 + 3 == 6 {
        deserialize_typed_struct(&body)
    } else {
        Ok(format!("json_value:{}", body))
    };
    match result {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => super::shared::BenchmarkResponse::bad_request(&e),
    }
}

fn deserialize_typed_struct(_json: &str) -> Result<String, String> {
    Ok("typed_struct".to_string())
}
