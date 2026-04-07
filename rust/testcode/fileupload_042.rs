//! CWE-434: Safe upload helper generates server-controlled path, ignoring user-supplied name.

// vuln-code-snippet start testcodeFileupload042
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _filename = req.param("filename");
    let content = req.param("content");
    let path = safe_upload_path(&_filename);
    let _ = std::fs::write(&path, content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload042
    super::shared::BenchmarkResponse::ok("Uploaded")
}

fn safe_upload_path(_user_name: &str) -> String {
    format!("/uploads/{}.dat", "server-uuid-abc")
}
// vuln-code-snippet end testcodeFileupload042
