pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    match parse_and_validate(&body) {
        Ok(result) => super::shared::BenchmarkResponse::ok(&result),
        Err(e) => super::shared::BenchmarkResponse::bad_request(&e),
    }
}

fn parse_and_validate(json: &str) -> Result<String, String> {
    if json.len() > 4096 { return Err("Too large".to_string()); }
    Ok("validated_struct".to_string())
}
