//! CWE-434: Both extension allowlist and size limit enforced before file is saved.

// vuln-code-snippet start testcodeFileupload044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    if !["jpg", "png", "pdf"].contains(&ext.as_str()) {
        return super::shared::BenchmarkResponse::bad_request("Extension not allowed");
    }
    if content.len() > 5_242_880 {
        return super::shared::BenchmarkResponse::bad_request("Too large");
    }
    let safe_name = format!("upload_safe.{}", ext);
    let _ = std::fs::write(format!("/uploads/{}", safe_name), content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload044
    super::shared::BenchmarkResponse::ok("Saved")
}
// vuln-code-snippet end testcodeFileupload044
