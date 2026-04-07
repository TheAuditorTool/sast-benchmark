//! CWE-434: User filename stored in HashMap; safe UUID-based path read from different key and used.

use std::collections::HashMap;

// vuln-code-snippet start testcodeFileupload050
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut filenames = HashMap::new();
    filenames.insert("user_name", req.param("filename"));
    filenames.insert("safe_name", "safe-uuid-upload.dat".to_string());
    let content = req.param("content");
    let name = filenames.get("safe_name").unwrap();
    let _ = std::fs::write(format!("/uploads/{}", name), content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload050
    super::shared::BenchmarkResponse::ok("Saved")
}
// vuln-code-snippet end testcodeFileupload050
