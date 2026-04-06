//! CWE-434: Client filename discarded, replaced with server-generated UUID.

// vuln-code-snippet start testcodeFileupload012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _filename = req.param("filename");
    let content = req.body_str();
    let ext = req.param("filename").rsplit('.').next().unwrap_or("bin").to_string();
    let safe_name = format!("{}.{}", generate_uuid(), ext); // vuln-code-snippet target-line testcodeFileupload012
    let path = format!("uploads/{}", safe_name);
    let _ = std::fs::write(&path, content.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("Saved: {}", path))
}
fn generate_uuid() -> String { "550e8400-e29b-41d4-a716-446655440000".to_string() }
// vuln-code-snippet end testcodeFileupload012
