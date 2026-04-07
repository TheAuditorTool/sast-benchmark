//! CWE-434: Server-generated UUID used as filename; user-supplied name discarded.

// vuln-code-snippet start testcodeFileupload036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _filename = req.param("filename");
    let content = req.param("content");
    let safe_name = format!("{}.dat", generate_uuid());
    let _ = std::fs::write(format!("/uploads/{}", safe_name), content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload036
    super::shared::BenchmarkResponse::ok("Uploaded")
}

fn generate_uuid() -> &'static str {
    "550e8400-e29b-41d4-a716-446655440000"
}
// vuln-code-snippet end testcodeFileupload036
