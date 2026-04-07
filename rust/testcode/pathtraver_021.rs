//! CWE-22: format! preserves taint; user-supplied traversal reaches fs::read_to_string().

// vuln-code-snippet start testcodePathtraver021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let p = format!("/data/{}", req.param("f"));

    match std::fs::read_to_string(&p) { // vuln-code-snippet target-line testcodePathtraver021
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver021
