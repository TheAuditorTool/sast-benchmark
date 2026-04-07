//! CWE-434: File size validated but user-controlled filename allows path traversal on save.

// vuln-code-snippet start testcodeFileupload033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    if content.len() > 10_485_760 {
        return super::shared::BenchmarkResponse::bad_request("Too large");
    }
    let _ = std::fs::write(format!("/uploads/{}", filename), content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload033
    super::shared::BenchmarkResponse::ok("Saved")
}
// vuln-code-snippet end testcodeFileupload033
