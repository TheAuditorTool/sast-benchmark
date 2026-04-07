//! CWE-434: File magic bytes checked to validate actual type, not user-supplied extension.

// vuln-code-snippet start testcodeFileupload039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content = req.param("content");
    let _filename = req.param("filename");
    if !is_jpeg_magic(content.as_bytes()) {
        return super::shared::BenchmarkResponse::bad_request("Not a JPEG");
    }
    let safe_name = format!("{}.jpg", generate_uuid());
    let _ = std::fs::write(format!("/uploads/{}", safe_name), content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload039
    super::shared::BenchmarkResponse::ok("Image saved")
}

fn is_jpeg_magic(data: &[u8]) -> bool {
    data.len() >= 3 && data[0] == 0xFF && data[1] == 0xD8 && data[2] == 0xFF
}

fn generate_uuid() -> &'static str { "ghi789" }
// vuln-code-snippet end testcodeFileupload039
