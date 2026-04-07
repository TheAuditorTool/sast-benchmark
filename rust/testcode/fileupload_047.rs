//! CWE-434: User-supplied filename unconditionally overwritten with server-generated safe name.

// vuln-code-snippet start testcodeFileupload047
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut filename = req.param("filename");
    filename = "server-safe-upload.dat".to_string();
    let content = req.param("content");
    let _ = std::fs::write(format!("/uploads/{}", filename), content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload047
    super::shared::BenchmarkResponse::ok("Uploaded")
}
// vuln-code-snippet end testcodeFileupload047
