//! CWE-434: File with .php extension accepted and saved to web-accessible directory.

// vuln-code-snippet start testcodeFileupload023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    if !filename.is_empty() {
        let _ = std::fs::write(format!("/var/www/uploads/{}", filename), content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload023
        super::shared::BenchmarkResponse::ok("Stored")
    } else {
        super::shared::BenchmarkResponse::bad_request("No filename")
    }
}
// vuln-code-snippet end testcodeFileupload023
