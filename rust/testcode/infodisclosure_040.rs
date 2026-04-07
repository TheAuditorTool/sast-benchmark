//! CWE-209: Error response struct strips internal details before returning to client.

// vuln-code-snippet start testcodeInfodisclosure040
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    match db_query(&id) {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => {
            let safe = SafeError::from_internal(&e);
            super::shared::BenchmarkResponse::error(&safe.message) // vuln-code-snippet target-line testcodeInfodisclosure040
        }
    }
}

struct SafeError { message: String }
impl SafeError {
    fn from_internal(_e: &str) -> Self {
        SafeError { message: "An error occurred. Please try again.".to_string() }
    }
}

fn db_query(_id: &str) -> Result<String, String> { Ok("data".to_string()) }
// vuln-code-snippet end testcodeInfodisclosure040
