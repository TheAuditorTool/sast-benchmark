//! CWE-502: Payload size strictly limited to 64KB before deserialization begins.

// vuln-code-snippet start testcodeDeser036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    if body.len() > 65_536 {
        return super::shared::BenchmarkResponse::bad_request("Payload too large");
    }
    let result = deserialize_typed_struct(&body); // vuln-code-snippet target-line testcodeDeser036
    match result {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => super::shared::BenchmarkResponse::bad_request(&e),
    }
}

fn deserialize_typed_struct(_json: &str) -> Result<String, String> {
    Ok("struct parsed".to_string())
}
// vuln-code-snippet end testcodeDeser036
