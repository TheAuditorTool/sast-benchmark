//! CWE-434: Upload handler reads entire body without file size limit.

// vuln-code-snippet start testcodeFileupload003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.body_str(); // vuln-code-snippet target-line testcodeFileupload003
    let path = format!("uploads/{}", filename);
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("Saved {} bytes", content.len()))
}
// vuln-code-snippet end testcodeFileupload003
