//! CWE-434: Constant-folded condition always routes to safe UUID-named upload path.

// vuln-code-snippet start testcodeFileupload046
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _filename = req.param("filename");
    let content = req.param("content");
    let path = if 8 * 8 == 64 {
        "/uploads/safe-uuid.dat".to_string() // vuln-code-snippet target-line testcodeFileupload046
    } else {
        format!("/uploads/{}", _filename)
    };
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok("Done")
}
// vuln-code-snippet end testcodeFileupload046
