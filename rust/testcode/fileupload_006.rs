//! CWE-434: Content-Disposition filename used directly in file system path.

// vuln-code-snippet start testcodeFileupload006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let disposition = req.header("content-disposition");
    let filename = extract_filename(&disposition);
    let content = req.body_str();
    let path = format!("uploads/{}", filename); // vuln-code-snippet target-line testcodeFileupload006
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("Saved: {}", path))
}
fn extract_filename(header: &str) -> String {
    header.split("filename=").nth(1).unwrap_or("unknown").trim_matches('"').to_string()
}
// vuln-code-snippet end testcodeFileupload006
