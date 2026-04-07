//! CWE-22: Only the final path component (basename) is used; directory traversal is stripped.

// vuln-code-snippet start testcodePathtraver030
fn extract_filename(input: &str) -> Option<&str> {
    std::path::Path::new(input).file_name()?.to_str()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("file");

    let filename = match extract_filename(&input) {
        Some(f) => f,
        None => return super::shared::BenchmarkResponse::bad_request("Invalid filename"),
    };

    let safe_path = format!("/uploads/{}", filename);
    match std::fs::read_to_string(&safe_path) { // vuln-code-snippet target-line testcodePathtraver030
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver030
