//! CWE-434: File stored outside web root, served only through authenticated endpoint.

// vuln-code-snippet start testcodeFileupload016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.body_str();
    let path = format!("/var/data/secure_uploads/{}", filename); // vuln-code-snippet target-line testcodeFileupload016
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok("Saved outside web root")
}
// vuln-code-snippet end testcodeFileupload016
