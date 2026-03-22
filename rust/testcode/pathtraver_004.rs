//! CWE-22: User controls directory creation path.

// vuln-code-snippet start testcodePathtraver004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_path = req.param("dir");

    match std::fs::create_dir_all(&user_path) { // vuln-code-snippet target-line testcodePathtraver004
        Ok(_) => super::shared::BenchmarkResponse::ok("Directory created"),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver004
