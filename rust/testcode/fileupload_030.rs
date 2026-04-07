//! CWE-434: User filename added to upload queue and saved without extension validation.

// vuln-code-snippet start testcodeFileupload030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    let mut queue: Vec<String> = Vec::new();
    queue.push(filename);
    for name in &queue {
        let _ = std::fs::write(format!("/uploads/{}", name), content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload030
    }
    super::shared::BenchmarkResponse::ok("Queued")
}
// vuln-code-snippet end testcodeFileupload030
