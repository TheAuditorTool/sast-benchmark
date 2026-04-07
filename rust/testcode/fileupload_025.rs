//! CWE-434: User-supplied filename used without stripping path traversal sequences.

// vuln-code-snippet start testcodeFileupload025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    let dest = format!("/uploads/{}", filename);
    let _ = std::fs::write(&dest, content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload025
    super::shared::BenchmarkResponse::ok("Written")
}
// vuln-code-snippet end testcodeFileupload025
