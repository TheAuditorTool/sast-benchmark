//! CWE-434: Uploaded file saved using user-supplied filename without validation.

// vuln-code-snippet start testcodeFileupload021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let data = req.param("data");
    let path = format!("/uploads/{}", filename);
    let _ = std::fs::write(&path, data.as_bytes()); // vuln-code-snippet target-line testcodeFileupload021
    super::shared::BenchmarkResponse::ok("Uploaded")
}
// vuln-code-snippet end testcodeFileupload021
