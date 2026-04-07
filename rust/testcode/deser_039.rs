//! CWE-502: Typed deserialization followed by field validation rejects malformed input.

// vuln-code-snippet start testcodeDeser039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    match parse_and_validate(&body) {
        Ok(result) => super::shared::BenchmarkResponse::ok(&result), // vuln-code-snippet target-line testcodeDeser039
        Err(e) => super::shared::BenchmarkResponse::bad_request(&e),
    }
}

fn parse_and_validate(json: &str) -> Result<String, String> {
    if json.len() > 4096 { return Err("Too large".to_string()); }
    Ok("validated_struct".to_string())
}
// vuln-code-snippet end testcodeDeser039
