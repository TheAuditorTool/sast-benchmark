//! CWE-209: Error details logged server-side only; generic message returned to client.

// vuln-code-snippet start testcodeInfodisclosure036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    match db_query(&id) {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => {
            log_error(&format!("DB error: {:?}", e));
            super::shared::BenchmarkResponse::error("Service unavailable") // vuln-code-snippet target-line testcodeInfodisclosure036
        }
    }
}

fn db_query(_id: &str) -> Result<String, String> { Ok("data".to_string()) }
fn log_error(msg: &str) { eprintln!("[ERROR] {}", msg); }
// vuln-code-snippet end testcodeInfodisclosure036
