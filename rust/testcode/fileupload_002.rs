//! CWE-434: File extension checked but content-type and magic bytes not verified.

// vuln-code-snippet start testcodeFileupload002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.body_str();
    if !filename.ends_with(".jpg") && !filename.ends_with(".png") { // vuln-code-snippet target-line testcodeFileupload002
        return super::shared::BenchmarkResponse::bad_request("Only images allowed");
    }
    let path = format!("uploads/{}", filename);
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("Saved: {}", path))
}
// vuln-code-snippet end testcodeFileupload002
