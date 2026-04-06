//! CWE-434: Archive entry extracted using raw path without traversal validation.

// vuln-code-snippet start testcodeFileupload008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let entry_name = req.param("entry_name");
    let content = req.body_str();
    // Simulates: raw ZipFile::name() used as path -- allows ../
    let path = format!("extract/{}", entry_name); // vuln-code-snippet target-line testcodeFileupload008
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("Extracted: {}", path))
}
// vuln-code-snippet end testcodeFileupload008
