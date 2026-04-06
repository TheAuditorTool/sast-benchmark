//! CWE-434: Uploaded file saved using original client-provided filename.

// vuln-code-snippet start testcodeFileupload001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.body_str();
    let path = format!("uploads/{}", filename);
    let _ = std::fs::write(&path, content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload001
    super::shared::BenchmarkResponse::ok(&format!("Saved: {}", path))
}
// vuln-code-snippet end testcodeFileupload001
