//! CWE-22: PathBuf::join with an absolute user input resets the base, bypassing the prefix.

// vuln-code-snippet start testcodePathtraver024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let p = std::path::PathBuf::from("/base").join(req.param("f"));

    match std::fs::read_to_string(&p) { // vuln-code-snippet target-line testcodePathtraver024
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver024
