//! CWE-22: User-supplied value overwritten by hardcoded assignment before the file operation.

// vuln-code-snippet start testcodePathtraver034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut p = req.param("f");
    p = "default.txt".to_string();

    let path = format!("/files/{}", p);
    match std::fs::read_to_string(&path) { // vuln-code-snippet target-line testcodePathtraver034
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver034
