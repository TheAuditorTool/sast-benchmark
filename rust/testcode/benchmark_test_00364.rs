pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    if 1 == 1 {
        if body.len() > 65_536 {
            return super::shared::BenchmarkResponse::bad_request("Too large");
        }
        let result = deserialize_typed_struct(&body);
        match result {
            Ok(r) => super::shared::BenchmarkResponse::ok(&r),
            Err(e) => super::shared::BenchmarkResponse::bad_request(&e),
        }
    } else {
        let v = parse_json_value(&body);
        super::shared::BenchmarkResponse::ok(&v)
    }
}

fn deserialize_typed_struct(_json: &str) -> Result<String, String> { Ok("typed".to_string()) }
fn parse_json_value(_json: &str) -> String { "value".to_string() }
