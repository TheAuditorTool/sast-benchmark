//! CWE-434: Image upload saved without verifying magic bytes match declared type.

// vuln-code-snippet start testcodeFileupload007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content_type = req.header("content-type");
    let content = req.body_str();
    if content_type.starts_with("image/") {
        let path = format!("uploads/images/{}", filename);
        let _ = std::fs::write(&path, content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload007
        super::shared::BenchmarkResponse::ok(&format!("Image saved: {}", path))
    } else {
        super::shared::BenchmarkResponse::bad_request("Only images allowed")
    }
}
// vuln-code-snippet end testcodeFileupload007
