//! CWE-434: Content-Type header trusted without verifying actual file content signature.

// vuln-code-snippet start testcodeFileupload027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content_type = req.header("Content-Type");
    let filename = req.param("filename");
    let content = req.param("content");
    if content_type.starts_with("image/") {
        let _ = std::fs::write(format!("/uploads/{}", filename), content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload027
        super::shared::BenchmarkResponse::ok("Image saved")
    } else {
        super::shared::BenchmarkResponse::bad_request("Not an image")
    }
}
// vuln-code-snippet end testcodeFileupload027
