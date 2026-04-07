//! CWE-434: Extension check vulnerable to null-byte injection in filename.

// vuln-code-snippet start testcodeFileupload032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    if filename.ends_with(".jpg") {
        let _ = std::fs::write(format!("/uploads/{}", filename), content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload032
        super::shared::BenchmarkResponse::ok("Image saved")
    } else {
        super::shared::BenchmarkResponse::bad_request("JPG only")
    }
}
// vuln-code-snippet end testcodeFileupload032
