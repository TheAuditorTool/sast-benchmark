//! CWE-209: Generic error response includes only a server-generated request ID, no internal details.

// vuln-code-snippet start testcodeInfodisclosure035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    match db_query(&id) {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(_e) => super::shared::BenchmarkResponse::error("Internal error. Request ID: req-12345"), // vuln-code-snippet target-line testcodeInfodisclosure035
    }
}

fn db_query(_id: &str) -> Result<String, String> { Ok("data".to_string()) }
// vuln-code-snippet end testcodeInfodisclosure035
