//! CWE-434: Uploaded file scanned for malware before being persisted.

// vuln-code-snippet start testcodeFileupload015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content = req.body_str();
    if !clamav_scan(content.as_bytes()) { // vuln-code-snippet target-line testcodeFileupload015
        return super::shared::BenchmarkResponse::bad_request("Malware detected");
    }
    let _ = std::fs::write("uploads/file", content.as_bytes());
    super::shared::BenchmarkResponse::ok("Saved after scan")
}
fn clamav_scan(data: &[u8]) -> bool {
    // Simulates: ClamAV scan -- returns true if clean
    !data.is_empty()
}
// vuln-code-snippet end testcodeFileupload015
