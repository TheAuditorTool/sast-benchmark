//! CWE-434: Binary file content accepted and stored without type or content validation.

// vuln-code-snippet start testcodeFileupload034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    store_binary(filename.as_str(), content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload034
    super::shared::BenchmarkResponse::ok("Stored binary")
}

fn store_binary(name: &str, data: &[u8]) {
    let _ = std::fs::write(format!("/storage/{}", name), data);
}
// vuln-code-snippet end testcodeFileupload034
