//! CWE-502: Deserialized into struct with deny_unknown_fields; unexpected fields cause parse error.

// vuln-code-snippet start testcodeDeser041
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let result = strict_deserialize(&body); // vuln-code-snippet target-line testcodeDeser041
    match result {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => super::shared::BenchmarkResponse::bad_request(&e),
    }
}

fn strict_deserialize(_json: &str) -> Result<String, String> {
    Ok("StrictRequest { name: String }".to_string())
}
// vuln-code-snippet end testcodeDeser041
