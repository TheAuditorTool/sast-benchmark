//! CWE-434: File extension validated against explicit safe allowlist before saving.

// vuln-code-snippet start testcodeFileupload037
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    if !["jpg", "jpeg", "png", "gif", "pdf"].contains(&ext.as_str()) {
        return super::shared::BenchmarkResponse::bad_request("Extension not allowed");
    }
    let safe_name = format!("{}.{}", generate_uuid(), ext);
    let _ = std::fs::write(format!("/uploads/{}", safe_name), content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload037
    super::shared::BenchmarkResponse::ok("Saved")
}

fn generate_uuid() -> &'static str { "abc123" }
// vuln-code-snippet end testcodeFileupload037
