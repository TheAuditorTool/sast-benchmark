//! CWE-434: User filename concatenated via format! into storage path without sanitization.

// vuln-code-snippet start testcodeFileupload028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_dir = req.param("user_id");
    let filename = req.param("filename");
    let content = req.param("content");
    let path = format!("/uploads/{}/{}", user_dir, filename);
    let _ = std::fs::write(&path, content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload028
    super::shared::BenchmarkResponse::ok("Stored")
}
// vuln-code-snippet end testcodeFileupload028
