//! CWE-22: Hardcoded path used; user input is read but never reaches the file operation.

// vuln-code-snippet start testcodePathtraver037
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    match std::fs::read_to_string("/app/static/welcome.html") { // vuln-code-snippet target-line testcodePathtraver037
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver037
