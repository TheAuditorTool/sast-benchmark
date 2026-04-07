//! CWE-502: Constant expression always selects safe size-checked typed deserialization path.

// vuln-code-snippet start testcodeDeser047
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    if 1 == 1 {
        if body.len() > 65_536 {
            return super::shared::BenchmarkResponse::bad_request("Too large");
        }
        let result = deserialize_typed_struct(&body); // vuln-code-snippet target-line testcodeDeser047
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
// vuln-code-snippet end testcodeDeser047
