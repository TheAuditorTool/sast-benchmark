//! CWE-434: Upload destination inside web-servable static directory.

// vuln-code-snippet start testcodeFileupload004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.body_str();
    let path = format!("./static/uploads/{}", filename); // vuln-code-snippet target-line testcodeFileupload004
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("Saved: {}", path))
}
// vuln-code-snippet end testcodeFileupload004
