//! CWE-434: Only the basename of user-supplied path extracted, preventing path traversal.

// vuln-code-snippet start testcodeFileupload040
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    let basename = std::path::Path::new(&filename)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("upload");
    let _ = std::fs::write(format!("/uploads/{}", basename), content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload040
    super::shared::BenchmarkResponse::ok("Saved")
}
// vuln-code-snippet end testcodeFileupload040
