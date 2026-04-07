//! CWE-434: File saved without validating extension; executable files accepted.

// vuln-code-snippet start testcodeFileupload022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    save_upload(&filename, content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload022
    super::shared::BenchmarkResponse::ok("Saved")
}

fn save_upload(name: &str, data: &[u8]) {
    let _ = std::fs::write(format!("/var/uploads/{}", name), data);
}
// vuln-code-snippet end testcodeFileupload022
