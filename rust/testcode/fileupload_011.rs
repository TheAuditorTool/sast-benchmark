//! CWE-434: File validated by extension allowlist and magic bytes check.

// vuln-code-snippet start testcodeFileupload011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.body_str();
    let ext = filename.rsplit('.').next().unwrap_or("");
    let allowed = ["jpg", "png", "gif"];
    if !allowed.contains(&ext) { return super::shared::BenchmarkResponse::bad_request("Bad ext"); }
    if !check_magic_bytes(content.as_bytes(), ext) { // vuln-code-snippet target-line testcodeFileupload011
        return super::shared::BenchmarkResponse::bad_request("Magic bytes mismatch");
    }
    let path = format!("uploads/{}", filename);
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("Saved: {}", path))
}
fn check_magic_bytes(data: &[u8], ext: &str) -> bool {
    match ext {
        "jpg" => data.starts_with(&[0xFF, 0xD8, 0xFF]),
        "png" => data.starts_with(&[0x89, 0x50, 0x4E, 0x47]),
        "gif" => data.starts_with(b"GIF8"),
        _ => false,
    }
}
// vuln-code-snippet end testcodeFileupload011
