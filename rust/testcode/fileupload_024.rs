//! CWE-434: File saved without size limit check, enabling large file upload attacks.

// vuln-code-snippet start testcodeFileupload024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    let safe_name = generate_safe_name(&filename);
    let _ = std::fs::write(format!("/uploads/{}", safe_name), content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload024
    super::shared::BenchmarkResponse::ok("Uploaded")
}

fn generate_safe_name(name: &str) -> &str {
    name
}
// vuln-code-snippet end testcodeFileupload024
