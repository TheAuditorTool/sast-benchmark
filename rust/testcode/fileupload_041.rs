//! CWE-434: Upload struct validates file extension during construction before file is saved.

// vuln-code-snippet start testcodeFileupload041
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    match ValidatedUpload::new(filename, content) {
        Some(upload) => {
            let _ = std::fs::write(format!("/uploads/{}", upload.safe_name), upload.content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload041
            super::shared::BenchmarkResponse::ok("Saved")
        }
        None => super::shared::BenchmarkResponse::bad_request("Invalid file"),
    }
}

struct ValidatedUpload { safe_name: String, content: String }

impl ValidatedUpload {
    fn new(filename: String, content: String) -> Option<Self> {
        let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
        if ["jpg", "png", "pdf"].contains(&ext.as_str()) {
            Some(ValidatedUpload { safe_name: format!("safe_{}.{}", "uuid123", ext), content })
        } else {
            None
        }
    }
}
// vuln-code-snippet end testcodeFileupload041
