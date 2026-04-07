//! CWE-209: Error formatter discards internal error details and returns static safe message.

// vuln-code-snippet start testcodeInfodisclosure045
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    match db_query(&id) {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => {
            let msg = safe_error_msg(&e); // vuln-code-snippet target-line testcodeInfodisclosure045
            super::shared::BenchmarkResponse::error(msg)
        }
    }
}

fn safe_error_msg(_internal_err: &str) -> &'static str { "An error occurred" }
fn db_query(_id: &str) -> Result<String, String> { Ok("data".to_string()) }
// vuln-code-snippet end testcodeInfodisclosure045
