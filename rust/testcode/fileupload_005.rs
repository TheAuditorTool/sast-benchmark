//! CWE-434: Handler accepts any Content-Type without validation.

// vuln-code-snippet start testcodeFileupload005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let _content_type = req.header("content-type"); // vuln-code-snippet target-line testcodeFileupload005
    let content = req.body_str();
    let path = format!("uploads/{}", filename);
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("Saved: {}", path))
}
// vuln-code-snippet end testcodeFileupload005
