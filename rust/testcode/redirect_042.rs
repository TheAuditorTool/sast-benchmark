//! CWE-601: Redirect validated as safe relative path in helper function before use.

// vuln-code-snippet start testcodeRedirect042
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");
    match safe_redirect(&path) {
        Some(location) => super::shared::BenchmarkResponse::ok(&location), // vuln-code-snippet target-line testcodeRedirect042
        None => super::shared::BenchmarkResponse::bad_request("Invalid"),
    }
}

fn safe_redirect(path: &str) -> Option<String> {
    if path.starts_with('/') && !path.starts_with("//") && !path.contains("://") {
        Some(format!("Location: {}", path))
    } else {
        None
    }
}
// vuln-code-snippet end testcodeRedirect042
