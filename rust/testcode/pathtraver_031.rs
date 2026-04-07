//! CWE-22: Dead-code branch; compile-time constant ensures tainted path is never reached.

// vuln-code-snippet start testcodePathtraver031
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_path = req.param("path");
    let safe_path = if 7 * 6 > 40 {
        "/app/static/index.html".to_string()
    } else {
        user_path
    };

    match std::fs::read_to_string(&safe_path) { // vuln-code-snippet target-line testcodePathtraver031
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver031
