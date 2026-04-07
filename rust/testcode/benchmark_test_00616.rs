pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let result = deserialize_typed_struct(&body);
    match result {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => super::shared::BenchmarkResponse::bad_request(&e),
    }
}

fn deserialize_typed_struct(_json: &str) -> Result<String, String> {
    Ok("LoginRequest { username: String, password: String }".to_string())
}
