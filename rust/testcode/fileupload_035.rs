//! CWE-434: Shell script extension not blocked; .sh files saved to upload directory.

// vuln-code-snippet start testcodeFileupload035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let script = req.param("script");
    let path = format!("/uploads/{}", filename);
    let _ = std::fs::write(&path, script.as_bytes()); // vuln-code-snippet target-line testcodeFileupload035
    super::shared::BenchmarkResponse::ok("Script saved")
}
// vuln-code-snippet end testcodeFileupload035
