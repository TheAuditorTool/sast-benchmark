//! CWE-434: User filename stored in UploadRequest struct and passed to write without validation.

// vuln-code-snippet start testcodeFileupload026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let upload = UploadRequest {
        filename: req.param("filename"),
        content: req.param("content"),
    };
    save_file(&upload.filename, upload.content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload026
    super::shared::BenchmarkResponse::ok("Saved")
}

struct UploadRequest { filename: String, content: String }

fn save_file(name: &str, data: &[u8]) {
    let _ = std::fs::write(format!("/uploads/{}", name), data);
}
// vuln-code-snippet end testcodeFileupload026
