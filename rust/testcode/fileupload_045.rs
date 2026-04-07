//! CWE-434: Basename extracted from user path and extension validated before saving.

// vuln-code-snippet start testcodeFileupload045
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    let basename = std::path::Path::new(&filename)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file");
    let ext = basename.rsplit('.').next().unwrap_or("").to_lowercase();
    if !["jpg", "png"].contains(&ext.as_str()) {
        return super::shared::BenchmarkResponse::bad_request("Bad extension");
    }
    let safe = format!("safe_{}.{}", "uuid", ext);
    let _ = std::fs::write(format!("/uploads/{}", safe), content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload045
    super::shared::BenchmarkResponse::ok("OK")
}
// vuln-code-snippet end testcodeFileupload045
